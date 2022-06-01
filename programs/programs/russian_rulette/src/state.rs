use anchor_lang::prelude::*;
use std::str::FromStr;

#[account]
pub struct RussianRulette {
    pub authority: Pubkey,
    pub random_oracle: Pubkey, // Oracle to retrieve random number
    pub players_idx: u32, // The idx of players participating
    pub ticket_price: u64
    pub bullet: u8
}

#[account]
#[derive(Default)]
pub struct Ticket{
    pub player: Pubkey,
    pub player_index: u8
}

#[account]
#[derive(Default)]
pub struct Winner{
    pub player: Pubkey,
    pub player_index: u8
}