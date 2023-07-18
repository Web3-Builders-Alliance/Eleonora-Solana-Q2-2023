use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::state::{Prediction, Game};

#[derive(Accounts)]
#[instruction(result: u8)]
pub struct Claim<'info> { 
    #[account(
        seeds = [
            creator.key().as_ref(),
            game.id.to_le_bytes().as_ref()
        ],
        bump = game.bump
    )]
    pub game: Account<'info, Game>,

    #[account(
        seeds = [
            game.key().as_ref(),
            &[result]
        ],
        bump
    )]
    pub prediction: Account<'info, Prediction>,

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
        init_if_needed,
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


impl<'info> Claim<'info> {
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
