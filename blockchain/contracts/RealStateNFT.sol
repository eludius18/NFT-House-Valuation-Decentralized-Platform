// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract RealEstateNFT is ERC721URIStorage {
    uint256 public tokenCounter;

    constructor() ERC721("RealEstateNFT", "HOUSE") {
        tokenCounter = 0;
    }

    function mintNFT(address to, string memory tokenURI) public returns (uint256) {
        uint256 tokenId = tokenCounter;
        tokenCounter++;
        _safeMint(to, tokenId);
        _setTokenURI(tokenId, tokenURI);
        return tokenId;
    }

    function updateMetadata(uint256 tokenId, string memory tokenURI) public {
        require(_isOwnerOrApproved(msg.sender, tokenId), "Not authorized to update metadata");
        _setTokenURI(tokenId, tokenURI);
    }

    function _isOwnerOrApproved(address spender, uint256 tokenId) internal view returns (bool) {
        address owner = ownerOf(tokenId);
        return (spender == owner || getApproved(tokenId) == spender || isApprovedForAll(owner, spender));
    }
}
