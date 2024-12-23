use anchor_lang::prelude::*;
use crate::{
    state::*,
    events::*,
};

pub fn process_pause(ctx: Context<CollectionAuthority>) -> Result<()> {
    let collection_state = &mut ctx.accounts.collection_state;
    
    // verify caller is collection authority
    collection_state.validate_authority(&ctx.accounts.authority.key())?;
    
    collection_state.paused = true;
    
    emit!(CollectionPaused {
        authority: ctx.accounts.authority.key(),
        collection_mint: collection_state.collection_mint,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

pub fn process_unpause(ctx: Context<CollectionAuthority>) -> Result<()> {
    let collection_state = &mut ctx.accounts.collection_state;
    
    // verify caller is collection authority
    collection_state.validate_authority(&ctx.accounts.authority.key())?;
    
    collection_state.paused = false;
    
    emit!(CollectionUnpaused {
        authority: ctx.accounts.authority.key(),
        collection_mint: collection_state.collection_mint,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}

