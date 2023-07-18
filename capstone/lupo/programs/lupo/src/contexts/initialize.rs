use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

use crate:: state::Global;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [
            admin.key().as_ref(),
            id.to_le_bytes().as_ref()
        ],
        payer = admin,
        bump,
        space = Global::LEN
    )]
    pub global: Account<'info, Global>,

    #[account(seeds = [b"auth", global.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [
            b"vault_dao",
            global.key().as_ref(),
            usdc_mint.key().as_ref(),
        ],
        token::mint = usdc_mint,
        token::authority = admin,
        bump
    )]
    pub vault_dao: Account<'info, TokenAccount>,
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
