use anchor_lang::prelude::*;
use anchor_spl::token::{self, TransferChecked};
use crate::{
    error::InterchangeableNFTError, events::*, state::*, utils::process_payment
};

pub fn pull_nft(ctx: Context<PullNft>) -> Result<()> {
    msg!("Starting pull NFT instruction");

    ctx.accounts.collection_state.check_not_paused()?;

    // Verify payment token matches
    require!(
        ctx.accounts.collection_state.payment_token_mint == ctx.accounts.payer_token_account.mint,
        InterchangeableNFTError::InvalidPaymentToken
    );

    process_payment(
        &ctx.accounts.token_program,
        &ctx.accounts.payer_token_account,
        &ctx.accounts.vault_token_account,
        &ctx.accounts.payer,
        ctx.accounts.collection_state.mint_price,
    )?;
    
    let collection_state = &mut ctx.accounts.collection_state;
    let nft_mint = &ctx.accounts.nft_mint;
    
    // Get collection state PDA seeds
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"collection",
        collection_state.authority.as_ref(),
        collection_state.collection_mint.as_ref(),
        &[collection_state.bump],
    ]];

    msg!("Executing NFT transfer");
    msg!("From: {}", ctx.accounts.vault_nft_account.key());
    msg!("To: {}", ctx.accounts.payer_nft_account.key());
    msg!("Authority: {}", collection_state.key());

    // Execute transfer
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_nft_account.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.payer_nft_account.to_account_info(),
            authority: collection_state.to_account_info(),
        },
        signer_seeds,
    );

    token::transfer_checked(transfer_ctx, 1, 0)?;  // 1 token, 0 decimals for NFT
    msg!("NFT transfer completed");

    // Send pull event
    emit!(NFTPull {
        puller: ctx.accounts.payer.key(),
        collection_mint: collection_state.collection_mint,
        mint: nft_mint.key(),
        amount: collection_state.mint_price,
        timestamp: Clock::get()?.unix_timestamp,
    });
    msg!("Pull event emitted");

    Ok(())
}