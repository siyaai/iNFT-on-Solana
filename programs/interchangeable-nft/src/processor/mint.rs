use anchor_lang::prelude::*;
use crate::{
    state::*,
    error::*,
    utils::process_payment,
};

use anchor_spl::token::Token;
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3, 
    CreateMetadataAccountV3InstructionArgs,
    VerifySizedCollectionItem,
    CreateMasterEditionV3,
    CreateMasterEditionV3InstructionArgs,
};

use solana_program::pubkey::Pubkey;
use crate::{events::*, metadata::*};

pub const METADATA_PROGRAM_PUBKEY: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

pub fn process_mint(ctx: Context<MintNFT>) -> Result<()> {
    // Check if collection is paused
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
    
    mint_single_nft(
        &mut ctx.accounts.collection_state,
        &ctx.accounts.payer,
        ctx.accounts.nft_mint.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.edition.to_account_info(),
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
        &ctx.accounts.rent,
        ctx.accounts.collection_metadata.to_account_info(),
        ctx.accounts.collection_master_edition.to_account_info(),
        ctx.accounts.collection_authority_record.to_account_info(),
        ctx.accounts.collection_mint.to_account_info(),
        ctx.accounts.pda_creator.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
    )
}


pub fn mint_single_nft<'info>(
    collection_state: &mut Account<'info, CollectionState>,
    payer: &Signer<'info>,
    nft_mint: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    edition: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    collection_metadata: AccountInfo<'info>,
    collection_master_edition: AccountInfo<'info>,
    collection_authority_record: AccountInfo<'info>,
    collection_mint: AccountInfo<'info>,
    pda_creator: AccountInfo<'info>,
    metadata_program: AccountInfo<'info>,
) -> Result<()> {
    msg!("=== Starting mint single NFT ===");

    // Check if exceeds max supply
    if collection_state.next_token_id >= collection_state.max_supply {
        return Err(InterchangeableNFTError::NoAvailableNFTs.into());
    }

    msg!("Creating new NFT, token_id: {}", collection_state.next_token_id);
    let token_id = collection_state.next_token_id;

    // Calculate PDA seeds and bump
    let collection_mint_key = collection_mint.key();
    let (pda_creator_calc, bump) = Pubkey::find_program_address(
        &[
            crate::ID.as_ref(),
            collection_mint_key.as_ref(),
            b"pda_creator",
        ],
        &crate::ID
    );

    let seeds = &[
        crate::ID.as_ref(),
        collection_mint_key.as_ref(),
        b"pda_creator" as &[u8],
        &[bump],
    ];

    // Compare pda_creator is correct
    require!(
        pda_creator_calc == pda_creator.key(),
        InterchangeableNFTError::PubkeyMismatch
    );
    
    // 1. Create NFT Metadata
    msg!("Creating NFT metadata");
    let metadata_data = create_nft_metadata_data(
        format!("{} #{}", collection_state.name, token_id),
        collection_state.symbol.clone(),
        format!("{}/{}", collection_state.base_uri, token_id),
        collection_state.royalty_config.basis_points,
        pda_creator.key(),
        collection_state.royalty_config.receiver,
        collection_state.collection_mint,
    );

    let args = CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: true,
        collection_details: None,
    };

    let cmv3 = CreateMetadataAccountV3 {
        metadata: metadata.key(),
        mint: nft_mint.key(),
        // mint_authority: pda_creator.key(),
        mint_authority: payer.key(),
        payer: payer.key(),
        update_authority: (pda_creator.key(), true),
        system_program: system_program.key(),
        rent: Some(rent.key()),
    };

    msg!("Creating metadata");

    solana_program::program::invoke_signed(
        &cmv3.instruction(args),
        &[
            metadata.to_account_info(),
            nft_mint.to_account_info(),
            pda_creator.to_account_info(),
            payer.to_account_info(),
            metadata_program.to_account_info(),
            token_program.to_account_info(),
            rent.to_account_info(),
            system_program.to_account_info(),
        ],
        &[seeds],
    )?;

    // 2. Create NFT Master Edition
    msg!("Creating NFT master edition");
    let create_master_edition_ix = CreateMasterEditionV3 {
        edition: edition.key(),
        mint: nft_mint.key(),
        update_authority: pda_creator.key(),
        mint_authority: payer.key(),
        metadata: metadata.key(),
        payer: payer.key(),
        token_program: token_program.key(),
        system_program: system_program.key(),
        rent: Some(rent.key()),
    };

    msg!("Master Edition accounts:");
    msg!("Edition: {}", edition.key());
    msg!("Mint: {}", nft_mint.key());
    msg!("Update Authority: {}", pda_creator.key());
    msg!("Mint Authority: {}", payer.key());
    msg!("Metadata: {}", metadata.key());

    solana_program::program::invoke_signed(
        &create_master_edition_ix.instruction(CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }),
        &[
            edition.to_account_info(),
            nft_mint.to_account_info(),
            pda_creator.to_account_info(),
            metadata.to_account_info(),
            payer.to_account_info(),
            metadata_program.to_account_info(),
            token_program.to_account_info(),
            system_program.to_account_info(),
            rent.to_account_info(),
        ],
        &[seeds],
    )?;

    // 3. Verify Collection
    msg!("Verifying sized collection item");
    let verify_collection = VerifySizedCollectionItem {
        metadata: metadata.key(),
        collection_authority: pda_creator.key(),
        payer: payer.key(),
        collection_mint: collection_mint.key(),
        collection: collection_metadata.key(),
        collection_master_edition_account: collection_master_edition.key(),
        collection_authority_record: Some(collection_authority_record.key()),
    };
    
    solana_program::program::invoke_signed(
        &verify_collection.instruction(),
        &[
            metadata.to_account_info(),
            pda_creator.to_account_info(),
            payer.to_account_info(),
            collection_mint.to_account_info(),
            collection_metadata.to_account_info(),
            collection_master_edition.to_account_info(),
            collection_authority_record.to_account_info(),
            metadata_program.to_account_info(), 
        ],
        &[seeds],
    )?;

    // 4. Send minted event
    emit!(NFTMinted {
        minter: payer.key(),
        collection_mint: collection_state.collection_mint,
        token_id: token_id,
        mint: nft_mint.key(),
        amount: collection_state.mint_price,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    collection_state.next_token_id += 1;
    Ok(())

}
