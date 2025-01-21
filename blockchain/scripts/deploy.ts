import { ethers } from "hardhat";

async function main() {
  console.log("Starting deployment...");

  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  const RealEstateNFT = await ethers.getContractFactory("RealEstateNFT");
  const contract = await RealEstateNFT.deploy({
      maxFeePerGas: ethers.utils.parseUnits("50", "gwei"), // Ajusta según sea necesario
      maxPriorityFeePerGas: ethers.utils.parseUnits("2", "gwei"), // Prioridad para el minero
  });

  await contract.deployed();
  console.log("RealEstateNFT deployed to:", contract.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
