use anchor_lang::prelude::*;


// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("8FaDrHMnbmGYcatRTqtnEtYqtRP5F7cKpEhFHqk3maiY");

mod errors;
mod state;
mod contexts;
mod constants;

use contexts::*;

#[program]
mod hello_anchor {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        ctx.accounts.global.bump = *ctx.bumps.get("global").unwrap();
        ctx.accounts.global.admin = *ctx.accounts.admin.key;
        Ok(())
    }

    pub fn create_game(ctx: Context<CreateGame>, title: String, id: u64) -> Result<()> {
        require!(title.len() < 33, errors::PredictionError::InvalidTitle);
        ctx.accounts.game.id = id;
        ctx.accounts.game.bump = *ctx.bumps.get("game").unwrap();
        ctx.accounts.game.result = 0xff; // x = TIE, 1 = WIN, 2 = LOSS, 255 = unresolved
        ctx.accounts.game.title = title;
        ctx.accounts.game.rate = 0;

        ctx.accounts.game.predictions[0]=0; //TIE
        ctx.accounts.game.predictions[1]=0; //WIN
        ctx.accounts.game.predictions[2]=0; //LOSE

        ctx.accounts.game.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.game.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.game.creator = *ctx.accounts.creator.key;
        Ok(())
    }

    pub fn finalize_game(ctx: Context<FinalizeGame>, result: u8, creator: Pubkey) -> Result<()> {
        require!(result < 3, errors::PredictionError::InvalidResult);
        require!(ctx.accounts.game.creator == creator, errors::PredictionError::NotCreator);
        //require signer to be the DAO contract
        ctx.accounts.game.result = result;
        // should move this to an external function that return a vault type depending on the result
        let winning_amount = if result == 0 {
            ctx.accounts.game.predictions[0]
        }
        else if result == 1 {
            ctx.accounts.game.predictions[1]
        }
        else {
            ctx.accounts.game.predictions[2]
        };


        // Should we use tokens instead of native sol?
        let total_deposited = ctx.accounts.game.predictions[0] + ctx.accounts.game.predictions[1] + ctx.accounts.game.predictions[2];
        let fee = 3 * total_deposited / 100 ; // TODO how to use decimal into solana?
        ctx.accounts.game.rate = total_deposited  - fee / winning_amount;

        ctx.accounts.transfer_to_vault(fee);
        Ok(())
    }

    pub fn make_prediction(ctx: Context<MakePrediction>, amount: u64, result: u8) -> Result<()> {
        require!(ctx.accounts.game.result == 0xff, errors::PredictionError::InvalidResult);
        let current_bet = ctx.accounts.prediction.amount;
        ctx.accounts.prediction.amount = current_bet.checked_add(amount).ok_or(errors::PredictionError::Overflow)?;
        ctx.accounts.prediction.result = result;
        ctx.accounts.prediction.bump = *ctx.bumps.get("prediction").unwrap();
        ctx.accounts.prediction.player = *ctx.accounts.player.key;
        ctx.accounts.prediction.control = 0;

        if result == 0 {
            ctx.accounts.game.predictions[0] += ctx.accounts.prediction.amount
        }
        else if result == 1 {
            ctx.accounts.game.predictions[1] += ctx.accounts.prediction.amount
        }
        else {
            ctx.accounts.game.predictions[2] += ctx.accounts.prediction.amount
        };

        ctx.accounts.transfer_to_vault(amount);
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, player: Pubkey) -> Result<()> {
        
        require_keys_eq!(ctx.accounts.prediction.player, player, errors::PredictionError::PredictionPlayerNotEqual);
        require!(ctx.accounts.prediction.result == ctx.accounts.game.result && ctx.accounts.prediction.control == 0, errors::PredictionError::InvalidResultOrControl);
        // check if user already claimed
        ctx.accounts.prediction.control = 1;
        let winning_amount = ctx.accounts.game.rate * ctx.accounts.prediction.amount;

        // update variable saying that the user already claimed

        ctx.accounts.transfer_to_vault(winning_amount);

        Ok(())
    }
}