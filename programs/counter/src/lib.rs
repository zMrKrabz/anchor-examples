use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Simple counter program to show how to
// - initialize an account with default count
// - add functions to a program
// - restrict to only initializers for updating counter
#[program]
pub mod roulette {
	use super::*;

	pub fn initialize(ctx: Context<Initialize>, authority: Pubkey, count: u64) -> ProgramResult {
		let counter = &mut ctx.accounts.counter;
		counter.count = count;
		counter.authority = authority;
		Ok(())
	}

	pub fn increase(ctx: Context<Increase>) -> ProgramResult {
		let counter = &mut ctx.accounts.counter;
		counter.count += 1;
		Ok(())
	}

	pub fn decrease(ctx: Context<Decrease>) -> ProgramResult {
		let counter = &mut ctx.accounts.counter;
		counter.count -= 1;
		Ok(())
	}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
	#[account(init, payer = user, space = 8 + 8)]
	pub counter: Account<'info, Counter>,
	#[account(mut)]
	pub user: Signer<'info>,
	pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increase<'info> {
	#[account(mut, has_one = authority)]
	pub counter: Account<'info, Counter>,
	pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct Decrease<'info> {
	#[account(mut)]
	pub counter: Account<'info, Counter>,
	pub authority: Signer<'info>,
}

#[account]
pub struct Counter {
	pub count: u64,
	pub authority: Pubkey,
}