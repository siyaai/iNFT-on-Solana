use anchor_lang::prelude::*;

use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs, 
    CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs, ApproveCollectionAuthority,
};
use solana_program::{
    program::invoke, program::invoke_signed,
};

use crate::{
    constants::*, error::*, events::*, state::*, metadata::*,
};

pub fn process_initialize(
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
    msg!("=== Starting process_initialize ===");
    
    // Validate parameters
    msg!("Validating parameters...");
    require!(
        max_supply >= MIN_MAX_SUPPLY && max_supply <= MAX_MAX_SUPPLY,
        InterchangeableNFTError::InvalidMaxSupply
    );
    
    require!(
        base_uri.len() <= MAX_URI_LENGTH,
        InterchangeableNFTError::InvalidBaseURI
    );
    
    require!(
        royalty_fee_basis_points <= MAX_ROYALTY_BASIS_POINTS,
        InterchangeableNFTError::InvalidRoyalty
    );

    // Calculate PDA seeds and bump
    msg!("Calculating PDA...");
    let collection_mint_key = ctx.accounts.collection_mint.key();
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
        pda_creator_calc == ctx.accounts.pda_creator.key(),
        InterchangeableNFTError::PubkeyMismatch
    );

    // 1. Create Collection Metadata
    msg!("Creating collection metadata");
    let metadata_data = create_collection_metadata_data(
        collection_name.clone(),
        collection_symbol.clone(),
        collection_uri.clone(),
        royalty_fee_basis_points,
        ctx.accounts.authority.key(),
    );

    let cmv3_args = CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: true,
        collection_details: Some(mpl_token_metadata::types::CollectionDetails::V1 { 
            size: 0 
        }),
    };

    let cmv3 = CreateMetadataAccountV3 {
        metadata: ctx.accounts.collection_metadata.key(),
        mint: ctx.accounts.collection_mint.key(),
        // mint_authority: ctx.accounts.pda_creator.key(),
        mint_authority: ctx.accounts.authority.key(),
        payer: ctx.accounts.authority.key(),
        // update_authority: (ctx.accounts.pda_creator.key(), true),
        update_authority: (ctx.accounts.authority.key(), true),
        system_program: ctx.accounts.system_program.key(),
        rent: Some(ctx.accounts.rent.key()),
    };

    invoke(
        &cmv3.instruction(cmv3_args),
        &[
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            // ctx.accounts.pda_creator.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ]
    )?;

    // 2. Create Collection Master Edition
    msg!("Creating collection master edition");
    let cmev3 = CreateMasterEditionV3 {
        edition: ctx.accounts.master_edition.key(),
        mint: ctx.accounts.collection_mint.key(),
        // update_authority: ctx.accounts.pda_creator.key(),
        // mint_authority: ctx.accounts.pda_creator.key(),
        update_authority: ctx.accounts.authority.key(),
        mint_authority: ctx.accounts.authority.key(),
        payer: ctx.accounts.authority.key(),
        metadata: ctx.accounts.collection_metadata.key(),
        token_program: ctx.accounts.token_program.key(),
        system_program: ctx.accounts.system_program.key(),
        rent: Some(ctx.accounts.rent.key()),
    };

    invoke(
        &cmev3.instruction(CreateMasterEditionV3InstructionArgs {
            max_supply: Some(0),
        }),
        &[
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            // ctx.accounts.pda_creator.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ]
    )?;

    // 3. Set Collection Authority
    msg!("Setting collection authority");
    let approve_collection = ApproveCollectionAuthority {
        collection_authority_record: ctx.accounts.collection_authority_record.key(),
        new_collection_authority: ctx.accounts.pda_creator.key(),
        update_authority: ctx.accounts.authority.key(),
        payer: ctx.accounts.authority.key(),
        metadata: ctx.accounts.collection_metadata.key(),
        mint: ctx.accounts.collection_mint.key(),
        system_program: ctx.accounts.system_program.key(),
        rent: Some(ctx.accounts.rent.key()), 
    };

    msg!("Collection Authority: {}", ctx.accounts.authority.key());
    msg!("PDA Creator: {}", ctx.accounts.pda_creator.key());

    invoke_signed(
        &approve_collection.instruction(),
        &[
            ctx.accounts.collection_authority_record.to_account_info(),
            ctx.accounts.pda_creator.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.collection_metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[seeds], 
    )?;

    // 4. Initialize Collection State
    msg!("Initializing collection state");
    let collection_state = &mut ctx.accounts.collection_state;
    collection_state.authority = ctx.accounts.authority.key();
    collection_state.payment_token_mint = ctx.accounts.payment_token_mint.key();
    collection_state.mint_price = mint_price;
    collection_state.max_supply = max_supply;
    collection_state.collection_mint = ctx.accounts.collection_mint.key();
    collection_state.base_uri = base_uri.clone();
    collection_state.next_token_id = 0;
    collection_state.paused = false;
    collection_state.name = collection_name.clone();
    collection_state.symbol = collection_symbol.clone();
    collection_state.royalty_config.basis_points = royalty_fee_basis_points;
    collection_state.royalty_config.receiver = royalty_fee_receiver;
    collection_state.fee_receiver = Pubkey::try_from(FEE_RECEIVER)
        .map_err(|_| error!(InterchangeableNFTError::InvalidFeeReceiver))?;
    collection_state.redeem_fee = REDEEM_FEE_BPS;
    collection_state.bump = ctx.bumps.collection_state;

    collection_state.token_decimals = ctx.accounts.payment_token_mint.decimals;

    // 5. Emit Collection Initialized Event
    emit!(CollectionInitialized {
        authority: ctx.accounts.authority.key(),
        collection_mint: ctx.accounts.collection_mint.key(),
        payment_token_mint: ctx.accounts.payment_token_mint.key(),
        mint_price,
        max_supply,
        base_uri: base_uri.clone(),
        collection_name: collection_name.clone(),
        collection_symbol: collection_symbol.clone(),
        collection_uri: collection_uri.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
} 