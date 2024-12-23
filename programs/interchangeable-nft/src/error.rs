use anchor_lang::prelude::*;

#[error_code]
pub enum InterchangeableNFTError {
    // price
    #[msg("Invalid mint price")]
    InvalidMintPrice,
    
    #[msg("Invalid redeem price")]
    InvalidRedeemPrice,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Insufficient balance to pay redeem fee")]
    InsufficientFeeBalance,
    
    #[msg("Incorrect payment amount")]
    IncorrectPaymentAmount,

    // supply
    #[msg("Invalid max supply")]
    InvalidMaxSupply,
    
    #[msg("Max supply reached")]
    MaxSupplyReached,
    
    #[msg("Insufficient supply")]
    InsufficientSupply,
    
    #[msg("No available NFTs")]
    NoAvailableNFTs,

    // uri
    #[msg("Invalid base URI")]
    InvalidBaseURI,
 
    // authority
    #[msg("Program is paused")]
    ProgramPaused,
    
    #[msg("Only owner can perform this action")]
    OnlyOwner,
    
    #[msg("Invalid fee receiver")]
    InvalidFeeReceiver,
    
    #[msg("Invalid account owner")]
    InvalidOwner,
    
    #[msg("Invalid collection mint")]
    InvalidCollectionMint,
    
    #[msg("Public key mismatch")]
    PubkeyMismatch,

    // base
    #[msg("Invalid string length")]
    InvalidStringLength,
    
    #[msg("Value cannot be zero")]
    InvalidZeroValue,
    
    #[msg("Value exceeds maximum allowed")]
    ExceedMaxValue,
    
    #[msg("Value below minimum required")]
    BelowMinValue,
    
    #[msg("Invalid royalty")]
    InvalidRoyalty,

    #[msg("Invalid payment token")]
    InvalidPaymentToken,

    #[msg("Invalid collection NFT")]
    InvalidCollectionNFT,

    #[msg("Unverified collection")]
    UnverifiedCollection,

    #[msg("Invalid NFT creator")]
    InvalidNFTCreator,
}