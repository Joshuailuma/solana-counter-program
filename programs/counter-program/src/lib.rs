use anchor_lang::prelude::*;
declare_id!("jfP84tt2CCPtqCbrHvhCAPhq4WwLmpg94GsF85PFhED");

#[program]
pub mod counter_program {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        ctx.accounts.counter.count -= 1;
        Ok(())
    }
}

// Creates the account
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = authority, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//The Counter account should just have two fields, one that stores the current count as a u64 and one that stores the pubkey of the authority over this account.
#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}

// all we want to do is take a counter account and decrement the value stored in its counter field if its authority has signed the transaction
// no new accounts are created
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}
