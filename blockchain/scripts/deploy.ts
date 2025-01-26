import { ethers } from "hardhat";
import * as dotenv from "dotenv";

dotenv.config();

async function main() {
  const name = process.env.NFT_NAME || "RealEstateNFT";
  const symbol = process.env.NFT_SYMBOL || "HOUSE";

  console.log(`Deploying RealEstateNFT with Name: ${name}, Symbol: ${symbol}`);

  try {
    const RealEstateNFT = await ethers.getContractFactory("RealEstateNFT");
    console.log("Contract factory initialized.");

    const realEstateNFT = await RealEstateNFT.deploy(name, symbol);
    console.log("Deployment initiated.");

    // Use the target directly if deployTransaction is undefined
    if (realEstateNFT.address) {
      console.log("Contract Address (direct):", realEstateNFT.address);
    } else if (realEstateNFT.target) {
      console.log("Contract Address (target):", realEstateNFT.target);
    } else {
      throw new Error("Failed to retrieve contract address.");
    }
  } catch (error) {
    console.error("Error during deployment:", error.message);
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error("Unhandled error:", error.message);
    process.exit(1);
  });
