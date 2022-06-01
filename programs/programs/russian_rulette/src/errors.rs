use anchor_lang::error;

#[error]
pub enum ErrorCode {
    #[msg("The amount of users should be 6")]
    MaxUsersReached,
    #[msg("You are invalidated")]
    InvalidatedUser
}

