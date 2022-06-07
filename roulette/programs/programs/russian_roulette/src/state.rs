use anchor_lang::prelude::*;

#[account]
pub struct RussianRoulette {
    pub authority: Pubkey,
    pub players_idx: u8, // The idx of players participating
    pub ticket_price: u64,
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