use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    associated_token::AssociatedToken,
};

use crate::{
    state::collection::*, 
    error::InterchangeableNFTError,
    constants::FEE_RECEIVER,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub payment_token_mint: Account<'info, Mint>,
    
    /// CHECK: Initialized in the instruction
    #[account(mut)]
    pub collection_mint: UncheckedAccount<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<CollectionState>() + 200,
        seeds = [
            b"collection".as_ref(), 
            authority.key().as_ref(),
            collection_mint.key().as_ref()
        ],
        bump
    )]
    pub collection_state: Account<'info, CollectionState>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    
    /// CHECK: Metadata account that will be created
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: Master edition account that will be created
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: PDA creator account
    #[account(mut)]
    pub pda_creator: UncheckedAccount<'info>,

    /// CHECK: Collection authority record
    #[account(mut)]
    pub collection_authority_record: UncheckedAccount<'info>,

    /// CHECK: Fee receiver account
    #[account(
        constraint = fee_receiver.key() == Pubkey::try_from(FEE_RECEIVER).unwrap() @ InterchangeableNFTError::InvalidFeeReceiver
    )]
    pub fee_receiver: UncheckedAccount<'info>,

    /// CHECK: Fee receiver token account
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = payment_token_mint,
        associated_token::authority = fee_receiver,
    )]
    pub fee_receiver_token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(mut, seeds = [b"collection".as_ref(), collection_state.authority.as_ref(), collection_state.collection_mint.as_ref()], bump = collection_state.bump)]
    pub collection_state: Account<'info, CollectionState>,
    
    #[account(mut, constraint = payer_token_account.mint == collection_state.payment_token_mint, constraint = payer_token_account.owner == payer.key())]
    pub payer_token_account: Account<'info, TokenAccount>,
    
    #[account(mut, constraint = vault_token_account.mint == collection_state.payment_token_mint, constraint = vault_token_account.owner == collection_state.key())]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Validated in instruction
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,
    
    /// CHECK: Metadata account that will be created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: Edition account that will be created
    #[account(mut)]
    pub edition: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = nft_mint,
        associated_token::authority = payer,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Validated in instruction
    pub token_metadata_program: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub collection_mint: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    pub collection_master_edition: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    #[account(mut)]
    pub collection_authority_record: UncheckedAccount<'info>,

    /// CHECK: Validated in instruction
    #[account(
        mut,
        seeds = [
            crate::ID.as_ref(),
            collection_mint.key().as_ref(),
            b"pda_creator",
        ],
        bump,
        seeds::program = crate::ID  
    )]
    pub pda_creator: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RedeemNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            b"collection".as_ref(), 
            collection_state.authority.as_ref(),
            collection_state.collection_mint.as_ref() 
        ],
        bump = collection_state.bump,
    )]
    pub collection_state: Account<'info, CollectionState>,
    
    pub nft_mint: Account<'info, Mint>,
    
    pub payment_token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = payer_nft_account.mint == nft_mint.key(),
        constraint = payer_nft_account.owner == payer.key(),
    )]
    pub payer_nft_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = vault_nft_account.mint == nft_mint.key(),
        constraint = vault_nft_account.owner == collection_state.key(),
    )]
    pub vault_nft_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = payer_token_account.mint == collection_state.payment_token_mint,
        constraint = payer_token_account.owner == payer.key(),
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = vault_token_account.mint == collection_state.payment_token_mint,
        constraint = vault_token_account.owner == collection_state.key(),
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: This is the fee receiver account
    #[account(
        mut,
        constraint = fee_receiver.key() == Pubkey::try_from(FEE_RECEIVER).unwrap() @ InterchangeableNFTError::InvalidFeeReceiver
    )]
    pub fee_receiver: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = fee_receiver_token_account.mint == collection_state.payment_token_mint,
        constraint = fee_receiver_token_account.owner == fee_receiver.key(),
    )]
    pub fee_receiver_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    /// CHECK: This is nft metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = mpl_token_metadata::ID,
        bump
    )]
    pub nft_metadata: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct PullNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            b"collection".as_ref(), 
            collection_state.authority.as_ref(),
            collection_state.collection_mint.as_ref()
        ],
        bump = collection_state.bump,
    )]
    pub collection_state: Account<'info, CollectionState>,
    
    // NFT 
    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = payer,
    )]
    pub payer_nft_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = collection_state,
    )]
    pub vault_nft_account: Account<'info, TokenAccount>,
    
    // payer account
    #[account(
        mut,
        associated_token::mint = collection_state.payment_token_mint,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = collection_state.payment_token_mint,
        associated_token::authority = collection_state,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [
            b"collection".as_ref(), 
            collection_state.authority.as_ref(),
            collection_state.collection_mint.as_ref()  
        ],
        bump = collection_state.bump,
        constraint = collection_state.authority == authority.key() @ InterchangeableNFTError::OnlyOwner
    )]
    pub collection_state: Account<'info, CollectionState>,
}

#[derive(Accounts)]
pub struct CollectionAuthority<'info> {
    #[account(mut)]
    pub collection_state: Account<'info, CollectionState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}