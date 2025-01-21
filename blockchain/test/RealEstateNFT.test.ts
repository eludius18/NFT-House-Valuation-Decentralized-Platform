import { expect } from "chai";
import { ethers } from "hardhat";

describe("RealEstateNFT", function () {
  let RealEstateNFT: any;
  let realEstateNFT: any;
  let owner: any, addr1: any;

  beforeEach(async () => {
    [owner, addr1] = await ethers.getSigners();
    RealEstateNFT = await ethers.getContractFactory("RealEstateNFT");
    realEstateNFT = await RealEstateNFT.deploy();
    await realEstateNFT.deployed();
  });

  it("Should mint an NFT and assign it to the owner", async function () {
    const tokenURI = JSON.stringify({
      name: "Luxury Villa",
      description: "A beautiful villa with 4 bedrooms.",
      attributes: [
        { trait_type: "Bedrooms", value: 4 },
        { trait_type: "Price", value: 500000 },
      ],
    });

    const mintTx = await realEstateNFT.mintNFT(owner.address, tokenURI);
    await mintTx.wait();

    expect(await realEstateNFT.ownerOf(0)).to.equal(owner.address);
    expect(await realEstateNFT.tokenURI(0)).to.equal(tokenURI);
  });

  it("Should allow metadata updates by the owner", async function () {
    const initialTokenURI = JSON.stringify({
      name: "Luxury Villa",
      description: "A beautiful villa with 4 bedrooms.",
      attributes: [
        { trait_type: "Bedrooms", value: 4 },
        { trait_type: "Price", value: 500000 },
      ],
    });

    const updatedTokenURI = JSON.stringify({
      name: "Luxury Villa - Updated",
      description: "An updated villa description.",
      attributes: [
        { trait_type: "Bedrooms", value: 5 },
        { trait_type: "Price", value: 600000 },
      ],
    });

    // Mint NFT
    const mintTx = await realEstateNFT.mintNFT(owner.address, initialTokenURI);
    await mintTx.wait();

    // Update metadata
    const updateTx = await realEstateNFT.updateMetadata(0, updatedTokenURI);
    await updateTx.wait();

    expect(await realEstateNFT.tokenURI(0)).to.equal(updatedTokenURI);
  });
});