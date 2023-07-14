use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::PredictionError,
    state::{Game, Global, Prediction},
};

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
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init,
        payer = creator,
        seeds = [
            b"vault",
            game.key().as_ref(),
            ID.as_ref(),
            mint.key().as_ref(),
        ]
    )]
    pub vault: Account<'info, TokenAccount>,
    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}
