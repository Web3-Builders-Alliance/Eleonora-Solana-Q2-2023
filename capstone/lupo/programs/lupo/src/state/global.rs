use anchor_lang::prelude::*;
use crate::constants::{ANCHOR_DISCRIMINATOR_BYTES, PUBKEY_BYTES, U8_BYTES};

#[account]
pub struct Global {
    pub admin: Pubkey,
    pub bump: u8,
}

impl Global {
    pub const LEN:usize = ANCHOR_DISCRIMINATOR_BYTES + PUBKEY_BYTES + U8_BYTES;
}