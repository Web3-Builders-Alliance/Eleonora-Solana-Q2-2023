use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::{state::{Game, Prediction, Global}, errors::PredictionError};


#[derive(Accounts)]
#[instruction(result: u8)]
pub struct FinalizeGame<'info> { 

    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = creator_token,
        associated_token::authority = creator
    )]
    pub creator_ata: Account<'info, TokenAccount>,
    pub creator_token: Box<Account<'info, Mint>>,

    #[account(
        seeds = [b"auth"],
        bump = game.bump
    )]

    pub auth: UncheckedAccount<'info>,
    #[account(
        payer = creator,
        seeds = [b"vault", game.key().as_ref()],
        bump,
        token::mint = creator_token,
        token::authority = auth
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub player: Account<'info, TokenAccount>
}


impl<'info> FinalizeGame<'info> {
    pub fn transfer_to_vault(
        &self,
        amount: u64
    ) -> Result<()> {

        let cpi_accounts = Transfer {
            from: self.creator_ata.to_account_info(),
            to: self.player.to_account_info(),
            authority: self.creator.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
}
