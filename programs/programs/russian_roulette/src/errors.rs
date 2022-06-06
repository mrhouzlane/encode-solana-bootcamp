use anchor_lang::error_code;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MaxUsersReached {}
#[derive(Accounts)]
pub struct InvalidatedUser {}

#[error_code]
pub enum ErrorCode {
    #[msg("The amount of users should be 6")]
    MaxUsersReached,
    #[msg("You are invalidated")]
    InvalidatedUser
}