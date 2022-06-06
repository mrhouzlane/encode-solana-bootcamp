use anchor_lang::prelude::*;
use errors::ErrorCode;
use crate::state::{
    RussianRoulette,
    Ticket,
    Winner,
};

mod errors;
mod state;

const MAX_PLAYERS: u8 = 7;
const NO_TICKETS: u8 = 0;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod russian_roulette {
    
    use super::*;

    pub fn create_game(ctx: Context<InitializeGame>, ticket_price: u64) -> Result<()> {

        let russian_roulette: &mut Account<RussianRoulette> = &mut ctx.accounts.russian_roulette;
        russian_roulette.authority = ctx.accounts.owner.key();
        //russian_roulette.winner = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
        russian_roulette.players_idx = NO_TICKETS; // 0 is the value selected to say that there are no player
        russian_roulette.ticket_price = ticket_price;
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        
        let russian_roulette: &mut Account<RussianRoulette> = &mut ctx.accounts.russian_roulette;
        let payer: &mut Signer = &mut ctx.accounts.player;
        let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;

        // We prepare the transaction
        let payment_ix = anchor_lang::solana_program::system_instruction::transfer(
            &payer.key(),
            &russian_roulette.key(),
            russian_roulette.ticket_price
        );

        // We make the transaction effective
        anchor_lang::solana_program::program::invoke(
            &payment_ix,
            &[payer.to_account_info(), russian_roulette.to_account_info()]
        )?;

        ticket.player = payer.key();

        if russian_roulette.players_idx < MAX_PLAYERS {
            ticket.player_index  = russian_roulette.players_idx + 1; // Starting from 1
            russian_roulette.players_idx += 1;
            Ok(())
        }
        else{
            Err(error!(ErrorCode::MaxUsersReached))
        };

        Ok(())
    }

    pub fn pull_trigger(ctx: Context<Shot>) -> Result<()>
    {
        // You can't pull the trigger if you have an invalid ticked or the game is over
        if ctx.accounts.ticket.player_index == NO_TICKETS {

            Err(error!(ErrorCode::InvalidatedUser))
        
        } else {

            let russian_roulette: &mut Account<RussianRoulette> = &mut ctx.accounts.russian_roulette;
            let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;

            //Randomness source: Node clock (Unsecure - Not 'so' random)
            let mut now_ts: i64 = Clock::get().unwrap().unix_timestamp;
            let mask: i64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
            now_ts = now_ts & mask;

            if now_ts == 1 {
                ticket.player_index = NO_TICKETS; // Invalidate ticket
                msg!("You have been shot. You loose.");
                russian_roulette.players_idx -= 1;
            }
            else {
                msg!("Empty shot. You continue playing!");
            };
            Ok(())    
        };
        Ok(())
    }

    pub fn transfer_reward(ctx: Context<Payout>) -> Result<()>{

        let russian_roulette: &mut Account<RussianRoulette> = &mut ctx.accounts.russian_roulette;
        let winner: &mut Account<Winner> = &mut ctx.accounts.winner;

        let rr_balance: u64 = russian_roulette.to_account_info().lamports();

        **russian_roulette.to_account_info().try_borrow_mut_lamports()? -= rr_balance;
        **winner.to_account_info().try_borrow_mut_lamports()? += rr_balance;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, payer = owner, space = 8 + 180)]
    pub russian_roulette: Account<'info, RussianRoulette>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System> // We need it because we are creating an account
}

#[derive(Accounts)]
pub struct WinnerContext<'info>{
    #[account(mut, constraint = russian_roulette.random_oracle == *oracle.key)]
    pub russian_roulette: Account<'info, RussianRoulette>,
    pub oracle: Signer<'info>
}
#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(init, 
        seeds = [
                &russian_roulette.players_idx.to_be_bytes(),
                russian_roulette.key().as_ref()
                ],
            bump,
            constraint = player.to_account_info().lamports() >= russian_roulette.ticket_price,
            payer = player,
            space = 200
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub russian_roulette: Account<'info, RussianRoulette>,
    pub system_program: Program<'info, System> // We need it because we are creating an account
}


#[derive(Accounts)]
pub struct Shot<'info>{
    #[account(mut,
    constraint = ticket.player_index <= russian_roulette.players_idx)]
    pub russian_roulette: Account<'info, RussianRoulette>,
    pub ticket: Account<'info, Ticket>
}

#[derive(Accounts)]
pub struct Payout<'info>{
    #[account(mut,
    constraint = winner.player == ticket.player &&
    ticket.player_index == winner.player_index)]
    pub russian_roulette: Account<'info, RussianRoulette>,
    
    pub winner: Account<'info, Winner>,
    pub ticket: Account<'info, Ticket>
}