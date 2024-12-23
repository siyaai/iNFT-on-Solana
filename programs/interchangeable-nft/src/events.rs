use anchor_lang::prelude::*;

#[event]
pub struct CollectionInitialized {
    pub authority: Pubkey,
    pub collection_mint: Pubkey,
    pub payment_token_mint: Pubkey,
    pub mint_price: u64,
    pub max_supply: u64,
    pub base_uri: String,
    pub collection_name: String,
    pub collection_symbol: String,
    pub collection_uri: String,
    pub timestamp: i64,
}

#[event]
pub struct NFTMinted {
    pub minter: Pubkey,
    pub collection_mint: Pubkey,
    pub token_id: u64,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct NFTRedeemed {
    pub redeemer: Pubkey,
    pub collection_mint: Pubkey,
    pub nft_mint: Pubkey,
    pub amount: u64,
    pub fee_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct CollectionPaused {
    pub authority: Pubkey,
    pub collection_mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct CollectionUnpaused {
    pub authority: Pubkey,
    pub collection_mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct NFTPull {
    pub puller: Pubkey,
    pub collection_mint: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
