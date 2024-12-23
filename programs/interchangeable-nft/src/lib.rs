use anchor_lang::prelude::*;

pub mod processor;
pub mod state;
pub mod events;
pub mod metadata;
pub mod error;
pub mod constants;
mod utils;

use state::*;

declare_id!("8BgkZFKRGHeRsQrKK8Jicv3CzqscgfvwRW6GqZ9yrXVn");

#[program]
pub mod interchangeable_nft {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        mint_price: u64,
        max_supply: u64,
        base_uri: String,
        collection_name: String,
        collection_symbol: String,
        collection_uri: String,
        royalty_fee_basis_points: u16,
        royalty_fee_receiver: Pubkey,
    ) -> Result<()> {
        processor::initialize::process_initialize(
            ctx,
            mint_price,
            max_supply,
            base_uri,
            collection_name,
            collection_symbol,
            collection_uri,
            royalty_fee_basis_points,
            royalty_fee_receiver,
        )
    }

    pub fn mint_nft(ctx: Context<MintNFT>) -> Result<()> {
        processor::mint::process_mint(ctx)
    }

    pub fn pull_nft(ctx: Context<PullNft>) -> Result<()> {
        processor::pull::pull_nft(ctx)
    }

    pub fn redeem_nft(ctx: Context<RedeemNFT>) -> Result<()> {
        processor::redeem::process_redeem(ctx)
    }

    pub fn pause(ctx: Context<CollectionAuthority>) -> Result<()> {
        processor::admin::process_pause(ctx)
    }

    pub fn unpause(ctx: Context<CollectionAuthority>) -> Result<()> {
        processor::admin::process_unpause(ctx)
    }

}      
