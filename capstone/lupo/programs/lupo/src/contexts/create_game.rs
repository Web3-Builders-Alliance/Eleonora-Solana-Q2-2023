use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

use crate:: state::Game;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateGame<'info> {
    #[account(
        init,
        seeds = [
            creator.key().as_ref(),
            id.to_le_bytes().as_ref()
        ],
        payer = creator,
        bump,
        space = Game::LEN
    )]
    pub game: Account<'info, Game>,
    #[account(seeds = [b"auth", game.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [
            b"vault",
            game.key().as_ref(),
            usdc_mint.key().as_ref(),
        ],
        token::mint = usdc_mint,
        token::authority = creator,
        bump
    )]
    pub vault: Account<'info, TokenAccount>,
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
