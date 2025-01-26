import { expect } from "chai";
import { ethers } from "hardhat";

describe("RealEstateNFT", function () {
  let RealEstateNFT: any;
  let realEstateNFT: any;
  let owner: any, addr1: any;

  beforeEach(async () => {
    [owner, addr1] = await ethers.getSigners();
    RealEstateNFT = await ethers.getContractFactory("RealEstateNFT");

    // Deploy the contract
    realEstateNFT = await RealEstateNFT.deploy("RealEstateNFT", "HOUSE");

    if (realEstateNFT.address) {
      console.log(`Contract Address: ${realEstateNFT.address}`);
    } else if (realEstateNFT.target) {
      console.log(`Contract Address (target): ${realEstateNFT.target}`);
    } else {
      throw new Error("Failed to deploy contract: address is undefined.");
    }
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

    await expect(mintTx)
      .to.emit(realEstateNFT, "NFTMinted")
      .withArgs(owner.address, ethers.BigNumber.from(0), tokenURI);

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

    const mintTx = await realEstateNFT.mintNFT(owner.address, initialTokenURI);

    const updateTx = await realEstateNFT.updateMetadata(0, updatedTokenURI);

    await expect(updateTx)
      .to.emit(realEstateNFT, "MetadataUpdated")
      .withArgs(ethers.BigNumber.from(0), updatedTokenURI);

    expect(await realEstateNFT.tokenURI(0)).to.equal(updatedTokenURI);
  });

  it("Should not allow unauthorized users to update metadata", async function () {
    const tokenURI = JSON.stringify({
      name: "Luxury Villa",
      description: "A beautiful villa with 4 bedrooms.",
      attributes: [
        { trait_type: "Bedrooms", value: 4 },
        { trait_type: "Price", value: 500000 },
      ],
    });

    const mintTx = await realEstateNFT.mintNFT(owner.address, tokenURI);

    const updatedTokenURI = JSON.stringify({
      name: "Luxury Villa - Updated",
      description: "An updated villa description.",
      attributes: [
        { trait_type: "Bedrooms", value: 5 },
        { trait_type: "Price", value: 600000 },
      ],
    });

    await expect(
      realEstateNFT.connect(addr1).updateMetadata(0, updatedTokenURI)
    ).to.be.reverted;
  });

  it("Should not allow minting by non-owners", async function () {
    const tokenURI = JSON.stringify({
      name: "Luxury Villa",
      description: "A beautiful villa with 4 bedrooms.",
      attributes: [
        { trait_type: "Bedrooms", value: 4 },
        { trait_type: "Price", value: 500000 },
      ],
    });

    await expect(
      realEstateNFT.connect(addr1).mintNFT(addr1.address, tokenURI)
    ).to.be.reverted;
  });
});