use anchor_lang::prelude::*;
use std::str::FromStr;
// use solana_program::{
//     account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
// };
mod state;


use crate::state::{
    RussianRulette,
    Ticket,
    Winner,
};

mod errors;

const MAX_PLAYERS: u32 = 6;

const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod russian_rulette {
    use super::*;


    pub fn create_game(ctx: Context<InitializeGame>, ticket_price: u64, random_oracle: Pubkey) -> Result<()> {

        let russian_rulette: &mut Account<RussianRulette> = &mut ctx.accounts.russian_rulette;
        russian_rulette.authority = ctx.accounts.owner.key();
        russian_rulette.winner = Pubkey::from_str(SYSTEM_PROGRAM).unwrap();
        russian_rulette.players = 0;
        russian_rulette.ticket_price = ticket_price;
        russian_rulette.random_oracle = random_oracle;
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()>{
        
        let russian_rulette: &mut Account<RussianRulette> = &mut ctx.accounts.russian_rulette;
        let payer: &mut Signer = &mut ctx.accounts.player;
        let ticket: &mut Account<Ticket> = &mut ctx.accounts.ticket;

        // We prepare the transaction
        let payment_ix = anchor_lang::solana_program::system_instruction::transfer(
            &payer.key(),
            &russian_rulette.key(),
            russian_rulette.ticket_price
        );

        // We make the transaction effective
        anchor_lang::solana_program::program::invoke(
            &payment_ix,
            &[payer.to_account_info(), russian_rulette.to_account_info()]
        )?;

        ticket.player = payer.key();

        if (russian_rulette.players_idx < MAX_PLAYERS ){
            ticket.player_index = russian_rulette.players_idx + 1;
            russian_rulette.players_idx += 1;
        }
        else{
            Err(MaxUsersReached)
        }
        
        Ok(())
    }

    pub fn pull_trigger(ctx: Context<Shot>) -> Result<()>
    {
        if ctx.ticket.player_index == -1 {
            Err(InvalidatedUser)
        }

        // You can't pull the trigger if you have an invalid ticked or the game is over
        let russian_rulette: &mut Account<RussianRulette> = ctx.accounts.russian_rulette;
        let ticket: &mut Account<Ticket> = ctx.accounts.ticket;

        check_status();

    }

    pub fn check_status(ticket: &mut Account<Ticket>)
    {
        /**
            use probability::prelude::*;

            let mut source = source::default();
            let distribution = Uniform::new(0.0, 1.0);
            let sampler = Independent(&distribution, &mut source);
            let samples = sampler.take(10).collect::<Vec<_>>();
        */
        
        if random(1, russian_roulette.players) == ticket.player_index {
            ctx.ticket.player_index = -1; // Invalidate ticket
        }

    }

    pub fn transfer_reward(ctx: Context<Payout>) -> Result<()>{

        let russian_rulette: &mut Account<RussianRulette> = ctx.accounts.russian_rulette;
        let winner: &mut Account<Winner> = ctx.accounts.winner;

        let rr_balance: u128 = russian_rulette.to_account_info().lamports();

        russian_rulette.to_account_info().try_borrow_mut_lamports()? -= rr_balance;
        winner.to_account_info().try_borrow_mut_lamports()? += rr_balance;

        Ok(())
    }

}



#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(init, payer = owner, space = 8 + 180)]
    pub russian_rulette: Account<'info, RussianRulette>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System> // We need it because we are creating an account
}

#[derive(Account)]
pub struct Winner<'infio>{
    #[account(mut, constraint = russian_rulette.random_oracle == oracle.key)]
    pub russian_rulette: Account<'info, RussianRulette>
    pub oracle: Signer<'info>
}
#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(init, 
        seeds = [
                &russian_rulette.players.to_be_bytes(),
                russian_rulette.key().as_ref()
                ],
            bump,
            constraint = player.to_account_info().lamports() >= russian_rulette.ticket_price,
            payer = player,
            space = 200
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub player: Signer<'info>,
    pub russian_rulette: Account<'info, RussianRulette>,
    pub system_program: Program<'info, System> // We need it because we are creating an account
}


#[derive(Accounts)]
pub struct Shot<'info>{
    #[account(mut,
    constraint = ticket.player_index <= russian_rulette.players)]
    pub russian_rulette: Account<'info, RussianRulette>,
    pub ticket: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Payout<'info>{
    #[account(mut,
    constraint = winner.player == ticket.player &&
    ticket.player_index == winner.player_index)]
    pub russian_rulette: Account<'info, RussianRulette>,
    
    pub winner: AccountInfo<'info>,
    pub ticket: Account<'info, Ticket>
}