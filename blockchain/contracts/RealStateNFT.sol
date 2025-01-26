// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/// @title RealEstateNFT
/// @notice This contract represents a platform for tokenizing real estate properties as NFTs.
/// @dev Implements the ERC721 standard and adds metadata management with ownership restrictions.
contract RealEstateNFT is ERC721, Ownable {
    /// @notice Counter to keep track of the next token ID to be minted.
    uint256 private _tokenCounter;

    /// @notice Mapping to store metadata URIs for each token.
    mapping(uint256 => string) private _tokenURIs;

    /// @notice Event emitted when a new NFT is minted.
    /// @param to The address that received the newly minted NFT.
    /// @param tokenId The unique identifier of the minted NFT.
    /// @param newTokenURI The metadata URI associated with the minted NFT.
    event NFTMinted(address indexed to, uint256 indexed tokenId, string newTokenURI);

    /// @notice Event emitted when the metadata of an existing NFT is updated.
    /// @param tokenId The unique identifier of the NFT.
    /// @param newTokenURI The new metadata URI associated with the NFT.
    event MetadataUpdated(uint256 indexed tokenId, string newTokenURI);

    /// @notice Constructor to initialize the NFT contract with a name and symbol.
    /// @param name_ The name of the token collection.
    /// @param symbol_ The symbol of the token collection.
    constructor(string memory name_, string memory symbol_) ERC721(name_, symbol_) Ownable(msg.sender) {
        _tokenCounter = 0;
    }

    /// @notice Mints a new NFT and assigns it to the specified address.
    /// @dev Only the owner of the contract can call this function.
    /// @param to The address that will own the minted NFT.
    /// @param newTokenURI The metadata URI associated with the NFT.
    /// @return tokenId The unique identifier of the minted NFT.
    function mintNFT(address to, string calldata newTokenURI) external onlyOwner returns (uint256) {
        uint256 tokenId = _tokenCounter;
        _tokenCounter++;

        // Mint the token and assign ownership
        _safeMint(to, tokenId);

        // Set the metadata URI for the token
        _setTokenURI(tokenId, newTokenURI);

        // Emit the event for minting
        emit NFTMinted(to, tokenId, newTokenURI);

        return tokenId;
    }

    /// @notice Updates the metadata URI for an existing NFT.
    /// @dev Only the owner of the NFT or an approved operator can call this function.
    /// @param tokenId The ID of the NFT to update.
    /// @param newTokenURI The new metadata URI to associate with the NFT.
    function updateMetadata(uint256 tokenId, string calldata newTokenURI) external {
        require(_exists(tokenId), "ERC721: Metadata update for nonexistent token");
        require(_isOwnerOrApproved(msg.sender, tokenId), "Unauthorized");

        // Update the token's metadata URI
        _setTokenURI(tokenId, newTokenURI);

        // Emit the event for metadata update
        emit MetadataUpdated(tokenId, newTokenURI);
    }

    /// @notice Retrieves the metadata URI for a specific NFT.
    /// @dev This function overrides the ERC721 implementation to include custom metadata storage.
    /// @param tokenId The ID of the NFT.
    /// @return The metadata URI associated with the NFT.
    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(_exists(tokenId), "ERC721: URI query for nonexistent token");
        return _tokenURIs[tokenId];
    }

    /// @dev Internal function to set the metadata URI for a specific NFT.
    ///      Ensures that the token exists before updating the URI.
    /// @param tokenId The ID of the NFT.
    /// @param newTokenURI The metadata URI to associate with the NFT.
    function _setTokenURI(uint256 tokenId, string memory newTokenURI) internal {
        require(_exists(tokenId), "ERC721: URI set for nonexistent token");
        _tokenURIs[tokenId] = newTokenURI;
    }

    /// @dev Internal function to check if a token exists.
    ///      Uses `_ownerOf` from the base ERC721 contract to determine existence.
    /// @param tokenId The ID of the NFT.
    /// @return True if the token exists, false otherwise.
    function _exists(uint256 tokenId) internal view returns (bool) {
        return _ownerOf(tokenId) != address(0);
    }

    /// @dev Internal function to verify if an address is the owner or an approved operator for a specific NFT.
    /// @param spender The address to verify.
    /// @param tokenId The ID of the NFT.
    /// @return True if the address is the owner or an approved operator, false otherwise.
    function _isOwnerOrApproved(address spender, uint256 tokenId) internal view returns (bool) {
        address owner = ownerOf(tokenId);
        return (spender == owner || getApproved(tokenId) == spender || isApprovedForAll(owner, spender));
    }
}