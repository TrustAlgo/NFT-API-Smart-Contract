// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract MyNFT is ERC721Enumerable, ERC721URIStorage, Ownable {

    uint256 private _nextTokenId;

    struct TokenDetails {
        string tokenName;
    }

    mapping(uint256 => TokenDetails) private _tokenDetails;

    constructor() ERC721("MyNFT", "MNFT") Ownable(msg.sender) {}

    // ==============================
    // Mint (Only Owner)
    // ==============================
    function mintNFT(
        address recipient,
        string memory tokenName,
        string memory tokenURI
    ) external onlyOwner returns (uint256) {
        uint256 tokenId = ++_nextTokenId;

        _safeMint(recipient, tokenId);
        _setTokenURI(tokenId, tokenURI);

        _tokenDetails[tokenId] = TokenDetails({
            tokenName: tokenName
        });

        return tokenId;
    }

    // ==============================
    // Get Token Details
    // ==============================
    function getTokenDetails(uint256 tokenId)
        external
        view
        returns (
            uint256,
            string memory,
            address,
            string memory
        )
    {
        require(_exists(tokenId), "Token does not exist");

        return (
            tokenId,
            _tokenDetails[tokenId].tokenName,
            ownerOf(tokenId),
            tokenURI(tokenId)
        );
    }

    // ==============================
    // Get Tokens By Owner
    // ==============================
    function getTokensByOwner(address owner)
        external
        view
        returns (uint256[] memory)
    {
        uint256 balance = balanceOf(owner);
        uint256[] memory tokenIds = new uint256[](balance);

        for (uint256 i = 0; i < balance; i++) {
            tokenIds[i] = tokenOfOwnerByIndex(owner, i);
        }

        return tokenIds;
    }

    // ==============================
    // Required Overrides
    // ==============================
    function _burn(uint256 tokenId)
        internal
        override(ERC721, ERC721URIStorage)
    {
        super._burn(tokenId);
        delete _tokenDetails[tokenId];
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721Enumerable)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
