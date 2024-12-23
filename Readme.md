# Interchangeable NFT

A Solana smart contract that combines the uniqueness of NFTs with the fungibility of tokens. This innovative protocol allows users to mint NFTs using tokens and redeem NFTs back to tokens, while maintaining each NFT's unique metadata and properties.

## Core Features

- **NFT-Token Hybrid System**: 
  - Mint any NFT from the collection for a fixed token price
  - Redeem your NFT back to tokens (with a 5% fee)
  - Pull specific NFTs from the collection vault using tokens
  - All NFTs within a collection have equal token value but unique metadata
- **Collection Management**: Initialize and manage NFT collections with configurable parameters
- **NFT Minting**: Mint NFTs with automatic metadata creation and collection verification
- **Token Integration**: 
  - Fixed token price for minting NFTs
  - Guaranteed token redemption value (minus fee)
  - Token-based NFT acquisition
- **Configurable Parameters**:
  - Fixed mint/redeem token price
  - Adjustable maximum supply (1-10,000)
  - Configurable royalties (up to 100%)
  - Fixed redeem fee (5%)
- **Metadata Integration**: Full integration with Metaplex Token Metadata Program
- **Payment System**: SPL Token integration for fixed-price operations
- **Admin Controls**: Pause/unpause functionality for collection operations

## How It Works

1. **Minting**: Users pay a fixed amount of tokens to mint any available NFT from the collection
2. **Redeeming**: Users can redeem their NFT back to tokens with a 5% fee
3. **Pulling**: Users can acquire specific NFTs from the vault by paying tokens
4. **Value Equality**: All NFTs within the same collection have equal token redemption value
5. **Unique Identity**: Each NFT maintains its unique metadata while having a fixed token value

## Technical Stack

- Solana Blockchain
- Anchor Framework 0.30.1
- Metaplex Token Metadata Program
- SPL Token Program

## Prerequisites

- Rust 1.70.0 or higher
- Solana CLI tools
- Anchor Framework 0.30.1
- Node.js 16+ (for testing)

## Installation

1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/interchangeable-nft-anchor.git
   cd interchangeable-nft-anchor
   ```

2. Install dependencies
   ```bash
   yarn install
   ```

3. Build the program
   ```bash
   anchor build
   ```

4. Deploy the program
   ```bash
   anchor deploy
   ```

## Usage

### Initialize Collection

Initialize a new NFT collection with the following parameters:
- Collection name and symbol
- Base URI for NFT metadata
- Maximum supply
- Fixed token price for mint/redeem
- Royalty configuration

```bash
anchor test tests/initialize-collection.ts
```

### Mint NFT

Pay tokens to mint a new NFT from the collection. The NFT will be automatically verified and added to the collection.

```bash
anchor test tests/mint-nft.ts
```

### NFT Operations

The program supports two main operations:
1. **Pull**: Pay tokens to get a specific NFT from the collection vault
2. **Redeem**: Convert your NFT back to tokens (minus 5% fee)

```bash
anchor test tests/swap-nft.ts
```

## Smart Contract Structure

```
programs/
└── interchangeable-nft/
    ├── src/
    │   ├── lib.rs           # Program entry point and instruction handlers
    │   ├── state/           # Program state and account structures
    │   ├── processor/       # Instruction processing logic
    │   ├── metadata.rs      # NFT metadata handling
    │   ├── events.rs        # Program events
    │   ├── error.rs         # Custom error types
    │   └── constants.rs     # Program constants
```

## Security Features

- Collection authority validation
- NFT metadata verification
- Collection verification checks
- Pause mechanism for emergency situations
- Secure token payment handling

## Configuration

Key parameters in `constants.rs`:
- Maximum URI length: 200 characters
- Maximum name length: 32 characters
- Maximum symbol length: 10 characters
- Maximum supply range: 1-10,000
- Fixed redeem fee: 5% (500 basis points)

## License

MIT License

## Contact

For questions and support, please open an issue in the repository.