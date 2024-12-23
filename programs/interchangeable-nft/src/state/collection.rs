use anchor_lang::prelude::*;
use crate::error::*;

#[account]
pub struct CollectionState {
    pub authority: Pubkey,
    pub collection_mint: Pubkey,
    pub payment_token_mint: Pubkey,
    pub mint_price: u64,
    pub max_supply: u64,
    pub next_token_id: u64,
    pub base_uri: String,
    pub paused: bool,
    pub bump: u8,
    pub name: String,
    pub symbol: String,
    pub fee_receiver: Pubkey,
    pub royalty_config: RoyaltyConfig,
    pub redeem_fee: u16,
    pub token_decimals: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RoyaltyConfig {
    pub basis_points: u16,
    pub receiver: Pubkey,
}

impl CollectionState {
    pub fn validate_authority(&self, authority: &Pubkey) -> Result<()> {
        require!(
            self.authority == *authority,
            InterchangeableNFTError::OnlyOwner
        );
        Ok(())
    }
    
    pub fn check_not_paused(&self) -> Result<()> {
        require!(!self.paused, InterchangeableNFTError::ProgramPaused);
        Ok(())
    }

    pub fn validate_mint_price(&self, payment_amount: u64) -> Result<()> {
        require!(
            payment_amount >= self.mint_price,
            InterchangeableNFTError::InvalidMintPrice
        );
        Ok(())
    }

} 