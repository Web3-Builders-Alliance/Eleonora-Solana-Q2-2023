use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_BYTES, PUBKEY_BYTES, U64_BYTES, U8_BYTES, STRING_BYTES};

#[account]
pub struct Prediction {
    player: Pubkey,
    result: u8,
    amount: u64,
    bump: u8,
    control: u64,
}

impl Prediction {
    pub const LEN:usize = ANCHOR_DISCRIMINATOR_BYTES + PUBKEY_BYTES + 2 * U8_BYTES + 2 * U64_BYTES;
}