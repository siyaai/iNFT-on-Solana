use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};


pub fn process_payment<'info>(
    token_program: &Program<'info, Token>,
    payer_token_account: &Account<'info, TokenAccount>,
    vault_token_account: &Account<'info, TokenAccount>,
    payer: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        token_program.to_account_info(),
        Transfer {
            from: payer_token_account.to_account_info(),
            to: vault_token_account.to_account_info(),
            authority: payer.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)
}