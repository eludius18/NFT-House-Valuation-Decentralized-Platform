use axum::{routing::post, Json, Router};
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::middleware::SignerMiddleware;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::Wallet;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::{env, sync::Arc};
use dotenv::dotenv;
use std::str::FromStr;

#[derive(Deserialize, Serialize)]
struct HouseDetails {
    name: String,
    bedrooms: u64,
    bathrooms: f64,
    sqft_living: u64,
    sqft_lot: u64,
    floors: u64,
    waterfront: u64,
    view: u64,
    condition: u64,
    grade: u64,
    sqft_above: u64,
    sqft_basement: u64,
    yr_built: u64,
    yr_renovated: u64,
    zipcode: u64,
    lat: f64,
    long: f64,
    sqft_living15: u64,
    sqft_lot15: u64,
    month: u64,
    year: u64,
}

#[derive(Serialize)]
struct MintResponse {
    transaction_hash: String,
    message: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    load_env_variables();

    let app = Router::new().route("/mint-nft", post(mint_nft));
    println!("Server running at http://localhost:3000...");
    if let Err(err) = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Server error: {}", err);
    }
}

fn load_env_variables() {
    let alchemy_url = env::var("ALCHEMY_URL").expect("ALCHEMY_URL is not set in .env");
    if !alchemy_url.starts_with("http://") && !alchemy_url.starts_with("https://") {
        panic!("ALCHEMY_URL must start with http:// or https://. Found: {}", alchemy_url);
    }
    println!("ALCHEMY_URL: {}", alchemy_url);

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set in .env");
    println!("PRIVATE_KEY: {}", if private_key.is_empty() { "None" } else { "Loaded" });

    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS is not set in .env");
    println!("CONTRACT_ADDRESS: {}", contract_address);
}

async fn mint_nft(Json(payload): Json<HouseDetails>) -> Result<Json<MintResponse>, String> {
    let python_url = "http://127.0.0.1:5000/predict";
    let client = Client::new();

    println!("Calling Python API for price prediction...");
    let response = client
        .post(python_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call Python API: {}", e))?;
    let price_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Python API response: {}", e))?;
    let price = price_data["price"]
        .as_f64()
        .ok_or("Price prediction missing or invalid in response")?;
    println!("Price prediction received: {}", price);

    let metadata = serde_json::json!({
        "name": payload.name,
        "description": format!("A {} bedroom house priced at ${}", payload.bedrooms, price),
        "attributes": [
            { "trait_type": "Bedrooms", "value": payload.bedrooms },
            { "trait_type": "Bathrooms", "value": payload.bathrooms },
            { "trait_type": "Living Area", "value": payload.sqft_living },
            { "trait_type": "Lot Size", "value": payload.sqft_lot },
            { "trait_type": "Price", "value": price }
        ]
    });

    println!("Connecting to Ethereum...");
    let alchemy_url = env::var("ALCHEMY_URL").expect("ALCHEMY_URL is not set in .env");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set in .env");
    let contract_address: Address = env::var("CONTRACT_ADDRESS")
        .expect("CONTRACT_ADDRESS is not set in .env")
        .parse()
        .expect("Invalid contract address");

    let provider = Provider::<Http>::try_from(alchemy_url).expect("Failed to connect to Ethereum provider");
    let wallet = Wallet::from_str(&private_key)
        .expect("Invalid private key")
        .with_chain_id(31337u64); // Hardhat's default chain ID
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    let abi: Abi = from_slice(include_bytes!("../abi/RealEstateNFT_abi.json"))
        .expect("Failed to load or parse the ABI file.");
    let contract = Contract::new(contract_address, abi, client.clone());

    println!("Preparing transaction to mint NFT...");
    let metadata_uri = serde_json::to_string(&metadata).expect("Failed to serialize metadata");
    let call = contract
        .method::<_, H256>("mintNFT", (client.address(), metadata_uri))
        .expect("Failed to create contract call");

    let pending_tx = call
        .send()
        .await
        .map_err(|e| format!("Failed to send transaction: {}", e))?;
    let receipt = pending_tx
        .await
        .map_err(|e| format!("Transaction failed: {}", e))?;

    let transaction_hash = receipt
        .ok_or("Transaction receipt is None")?
        .transaction_hash
        .to_string();

    println!("NFT minted successfully with transaction hash: {}", transaction_hash);

    Ok(Json(MintResponse {
        transaction_hash,
        message: "NFT minted successfully.".to_string(),
    }))
}