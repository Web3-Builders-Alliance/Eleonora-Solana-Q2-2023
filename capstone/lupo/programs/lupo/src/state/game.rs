use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_BYTES, PUBKEY_BYTES, U64_BYTES, U8_BYTES, STRING_BYTES};

#[account]
pub struct Game {
    pub id: u64,
    pub result: u8, //string
    pub bump: u8,
    pub title: String,
    pub rate: u64,

    pub creator: Pubkey,
    pub creator_token: Pubkey,
    pub seed: u64,
    pub auth_bump: u8,
    pub vault_bump: u8,

    pub predictions: Vec<u64>
}

impl Game {
    pub const LEN:usize = ANCHOR_DISCRIMINATOR_BYTES + 2 * PUBKEY_BYTES + 5 * U64_BYTES + 4 * U8_BYTES + STRING_BYTES;
}
