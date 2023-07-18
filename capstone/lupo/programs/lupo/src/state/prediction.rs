use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_BYTES, PUBKEY_BYTES, U64_BYTES, U8_BYTES};

#[account]
pub struct Prediction {
    pub player: Pubkey,
    pub result: u8,
    pub amount: u64,
    pub bump: u8,
    pub control: u64,
}

impl Prediction {
    pub const LEN:usize = ANCHOR_DISCRIMINATOR_BYTES + PUBKEY_BYTES + 2 * U8_BYTES + 2 * U64_BYTES;
}