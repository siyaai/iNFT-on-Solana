use anchor_lang::prelude::*;
use anchor_spl::token::{self, TransferChecked};

use crate::{
    error::InterchangeableNFTError, events::*, state::*
};

pub fn process_redeem(ctx: Context<RedeemNFT>) -> Result<()> {
    let collection_state = &mut ctx.accounts.collection_state;
    
    collection_state.check_not_paused()?;


    // 0.1 verify NFT Metadata
    let nft_metadata = mpl_token_metadata::accounts::Metadata::try_from(
        &ctx.accounts.nft_metadata.to_account_info()
    )?;
    require!(nft_metadata.collection.is_some(), InterchangeableNFTError::InvalidCollectionNFT);
    
    let collection = nft_metadata.collection.as_ref().unwrap();
    
    // 0.2 verify Collection
    require!(
        collection.key == collection_state.collection_mint,
        InterchangeableNFTError::InvalidCollectionNFT
    );
    require!(
        collection.verified,
        InterchangeableNFTError::UnverifiedCollection
    );

    // 0.3 verify Creator
    let creators = nft_metadata.creators.as_ref()
        .ok_or(InterchangeableNFTError::InvalidNFTCreator)?;
    
    let pda_creator = Pubkey::find_program_address(
        &[
            crate::ID.as_ref(),
            collection_state.collection_mint.as_ref(),
            b"pda_creator",
        ],
        &crate::ID
    ).0;

    require!(
        creators.iter().any(|c| c.address == pda_creator && c.verified),
        InterchangeableNFTError::InvalidNFTCreator
    );

    msg!("Transferring NFT from payer to vault");
    //  1. transfer Nft  transfer_checked
    let transfer_nft_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.payer_nft_account.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
            to: ctx.accounts.vault_nft_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::transfer_checked(transfer_nft_ctx, 1, 0)?;
    msg!("NFT transfer completed");


    // 2. Execute all transfers
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"collection",
        collection_state.authority.as_ref(),
        collection_state.collection_mint.as_ref(),
        &[collection_state.bump],
    ]];

    // 3. Calculate amount
    let (fee_amount, user_amount) = {
        let fee_amount = collection_state.mint_price
            .checked_mul(collection_state.redeem_fee as u64)
            .unwrap()
            .checked_div(10000)
            .unwrap();
        let user_amount = collection_state.mint_price
            .checked_sub(fee_amount)
            .unwrap();
        (fee_amount, user_amount)
    };

    // 4. Transfer fee to specified account using transfer_checked
    {
        let fee_transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.payment_token_mint.to_account_info(),
                to: ctx.accounts.fee_receiver_token_account.to_account_info(),
                authority: collection_state.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer_checked(
            fee_transfer_ctx, 
            fee_amount, 
            ctx.accounts.payment_token_mint.decimals
        )?;
        msg!("Fee transfer completed: {} tokens", fee_amount);

        // 5. Transfer remaining tokens to user using transfer_checked
        let user_transfer_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_token_account.to_account_info(),
                mint: ctx.accounts.payment_token_mint.to_account_info(),
                to: ctx.accounts.payer_token_account.to_account_info(),
                authority: collection_state.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer_checked(
            user_transfer_ctx, 
            user_amount, 
            ctx.accounts.payment_token_mint.decimals
        )?;
        msg!("User transfer completed: {} tokens", user_amount);
    };

    // 6. Send redeem event
    emit!(NFTRedeemed {
        redeemer: ctx.accounts.payer.key(),
        collection_mint: collection_state.collection_mint,
        nft_mint: ctx.accounts.nft_mint.key(),
        amount: collection_state.mint_price,
        fee_amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
} 