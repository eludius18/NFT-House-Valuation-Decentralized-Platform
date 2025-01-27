use axum::{routing::{get, post}, Json, Router};
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

/// Struct to represent the details of a house for NFT minting.
///
/// This struct holds all the information related to a house that will be used
/// to generate metadata and predict the price of the house.
///
/// # Fields
/// - `name`: Name of the property (e.g., "Luxury Villa").
/// - `bedrooms`: Number of bedrooms in the house.
/// - `bathrooms`: Number of bathrooms in the house (can be a fractional value).
/// - `sqft_living`: Size of the living area in square feet.
/// - `sqft_lot`: Size of the lot in square feet.
/// - `floors`: Number of floors in the house.
/// - `waterfront`: Indicates whether the property is near a waterfront (1 for yes, 0 for no).
/// - `view`: Quality of the view (represented as a numerical value).
/// - `condition`: Condition of the house (numeric scale).
/// - `grade`: Quality grade of the house (numeric scale).
/// - `sqft_above`: Square footage of the house excluding the basement.
/// - `sqft_basement`: Square footage of the basement area.
/// - `yr_built`: Year the house was built.
/// - `yr_renovated`: Year the house was last renovated.
/// - `zipcode`: The postal code of the property.
/// - `lat`: Latitude of the property.
/// - `long`: Longitude of the property.
/// - `sqft_living15`: Living area square footage of the nearest 15 neighbors.
/// - `sqft_lot15`: Lot size of the nearest 15 neighbors.
/// - `month`: Month in which the house sale is recorded.
/// - `year`: Year in which the house sale is recorded.
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

/// Struct for the response after minting an NFT.
///
/// This struct holds the transaction hash and a message indicating the result
/// of the minting process.
#[derive(Serialize)]
struct MintResponse {
    transaction_hash: String,
    message: String,
}

/// Struct for the metadata response when querying an NFT's metadata.
///
/// This struct contains the token ID and the metadata associated with that token.
/// The metadata is typically stored as a JSON object on the blockchain.
#[derive(Serialize, Deserialize)]
struct MetadataResponse {
    token_id: u64,
    metadata: serde_json::Value,
}

/// Entry point of the application, initializing the Axum server and defining routes.
///
/// This function sets up the server to listen for incoming requests and route them
/// to the appropriate handler functions (`/mint-nft` for minting NFTs and `/get-metadata`
/// for fetching metadata of a specific NFT by its token ID).
#[tokio::main]
async fn main() {
    dotenv().ok();  // Load environment variables from .env file
    load_env_variables();  // Load and validate environment variables

    // Set up Axum routes for minting NFT and retrieving metadata
    let app = Router::new()
        .route("/mint-nft", post(mint_nft))
        .route("/get-metadata/:token_id", get(get_metadata));

    println!("Server running at http://localhost:3000...");

    // Start the Axum server and handle errors if any occur
    if let Err(err) = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
    {
        eprintln!("Server error: {}", err);
    }
}

/// Loads the necessary environment variables from the .env file and validates them.
///
/// # Panics
/// This function will panic if the environment variables `ALCHEMY_URL`, `PRIVATE_KEY`,
/// or `CONTRACT_ADDRESS` are missing or invalid.
fn load_env_variables() {
    // Load and validate ALCHEMY_URL environment variable
    let alchemy_url = env::var("ALCHEMY_URL").expect("ALCHEMY_URL is not set in .env");
    if !alchemy_url.starts_with("http://") && !alchemy_url.starts_with("https://") {
        panic!("ALCHEMY_URL must start with http:// or https://. Found: {}", alchemy_url);
    }
    println!("ALCHEMY_URL: {}", alchemy_url);

    // Load PRIVATE_KEY environment variable
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set in .env");
    println!("PRIVATE_KEY: {}", if private_key.is_empty() { "None" } else { "Loaded" });

    // Load CONTRACT_ADDRESS environment variable
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS is not set in .env");
    println!("CONTRACT_ADDRESS: {}", contract_address);
}

/// Handles the minting of an NFT for a given house by interacting with the Python model
/// to predict the price and then calling the smart contract to mint the NFT.
///
/// # Parameters
/// - `payload`: JSON body containing the details of the house to mint as an NFT.
///
/// # Returns
/// - `Result<Json<MintResponse>, String>`: Returns the transaction hash and a success message upon successful minting,
///   or an error message if something goes wrong.
async fn mint_nft(Json(payload): Json<HouseDetails>) -> Result<Json<MintResponse>, String> {
    // Predict the house price using the Python model
    let python_url = "http://127.0.0.1:5000/predict";
    let client = Client::new();
    println!("Calling Python API for price prediction...");
    let response = client
        .post(python_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call Python API: {}", e))?;
    
    // Parse the predicted price from the response
    let price_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Python API response: {}", e))?;
    let price = price_data["price"]
        .as_f64()
        .ok_or("Price prediction missing or invalid in response")?;
    println!("Price prediction received: {}", price);

    // Create the metadata for the NFT
    let metadata = serde_json::json!({
        "name": payload.name,
        "description": format!("A {} bedroom house priced at ${}", payload.bedrooms, price),
        "attributes": [
            { "trait_type": "Bedrooms", "value": payload.bedrooms },
            { "trait_type": "Bathrooms", "value": payload.bathrooms },
            { "trait_type": "Living Area", "value": payload.sqft_living },
            { "trait_type": "Lot Size", "value": payload.sqft_lot },
            { "trait_type": "Floors", "value": payload.floors },
            { "trait_type": "Waterfront", "value": payload.waterfront },
            { "trait_type": "View", "value": payload.view },
            { "trait_type": "Condition", "value": payload.condition },
            { "trait_type": "Grade", "value": payload.grade },
            { "trait_type": "Sqft Above", "value": payload.sqft_above },
            { "trait_type": "Sqft Basement", "value": payload.sqft_basement },
            { "trait_type": "Year Built", "value": payload.yr_built },
            { "trait_type": "Year Renovated", "value": payload.yr_renovated },
            { "trait_type": "Zipcode", "value": payload.zipcode },
            { "trait_type": "Latitude", "value": payload.lat },
            { "trait_type": "Longitude", "value": payload.long },
            { "trait_type": "Living Area", "value": payload.sqft_living15 },
            { "trait_type": "Lot Size", "value": payload.sqft_lot15 },
            { "trait_type": "Month", "value": payload.month },
            { "trait_type": "Year", "value": payload.year },
            { "trait_type": "Price", "value": price }
        ]
    });

    // Prepare and send the transaction to mint the NFT on Ethereum
    println!("Connecting to Ethereum...");
    let alchemy_url = env::var("ALCHEMY_URL").expect("ALCHEMY_URL is not set in .env");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY is not set in .env");
    let contract_address: Address = env::var("CONTRACT_ADDRESS")
        .expect("CONTRACT_ADDRESS is not set in .env")
        .parse()
        .expect("Invalid contract address");

    let provider = Provider::<Http>::try_from(alchemy_url).expect("Failed to connect to Ethereum provider");
    let provider = Arc::new(provider);  // Use Arc to share the provider across multiple threads

    let wallet = Wallet::from_str(&private_key)
        .expect("Invalid private key")
        .with_chain_id(31337u64);
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    // Load the ABI for the RealEstateNFT contract
    let abi: Abi = from_slice(include_bytes!("../abi/RealEstateNFT_abi.json"))
        .expect("Failed to load or parse the ABI file.");
    let contract = Contract::new(contract_address, abi, provider.clone());

    // Mint the NFT by calling the smart contract method
    println!("Preparing transaction to mint NFT...");
    let metadata_uri = serde_json::to_string(&metadata).expect("Failed to serialize metadata");
    let call = contract
        .method::<_, H256>("mintNFT", (client.address(), metadata_uri))
        .expect("Failed to create contract call");

    // Send the transaction and wait for the receipt
    let pending_tx = call
        .send()
        .await
        .map_err(|e| format!("Failed to send transaction: {}", e))?;
    let receipt = pending_tx
        .await
        .map_err(|e| format!("Transaction failed: {}", e))?;

    // Extract the transaction hash and return it as part of the response
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

/// Fetches the metadata of an NFT based on its token ID.
///
/// This function queries the smart contract for the metadata associated with a
/// specific token ID and returns the metadata as a JSON response.
///
/// # Parameters
/// - `token_id`: The unique ID of the NFT whose metadata is to be fetched.
///
/// # Returns
/// - `Result<Json<MetadataResponse>, String>`: The metadata associated with the NFT
///   in a JSON format, or an error message if the metadata could not be retrieved.
async fn get_metadata(axum::extract::Path(token_id): axum::extract::Path<u64>) -> Result<Json<MetadataResponse>, String> {
    println!("Fetching metadata for token ID: {}", token_id);

    let alchemy_url = env::var("ALCHEMY_URL").expect("ALCHEMY_URL is not set in .env");
    let contract_address: Address = env::var("CONTRACT_ADDRESS")
        .expect("CONTRACT_ADDRESS is not set in .env")
        .parse()
        .expect("Invalid contract address");

    let provider = Provider::<Http>::try_from(alchemy_url).expect("Failed to connect to Ethereum provider");
    let provider = Arc::new(provider);  // Wrap the provider in an Arc to share across threads

    let abi: Abi = from_slice(include_bytes!("../abi/RealEstateNFT_abi.json"))
        .expect("Failed to load or parse the ABI file.");
    let contract = Contract::new(contract_address, abi, provider.clone());

    // Call the contract to get the metadata for the token ID
    let metadata: String = contract
        .method("tokenURI", token_id)
        .expect("Failed to create contract call")
        .call()
        .await
        .map_err(|e| format!("Failed to fetch metadata: {}", e))?;

    println!("Metadata fetched: {}", metadata);

    Ok(Json(MetadataResponse {
        token_id,
        metadata: serde_json::from_str(&metadata).unwrap_or_default(),
    }))
}