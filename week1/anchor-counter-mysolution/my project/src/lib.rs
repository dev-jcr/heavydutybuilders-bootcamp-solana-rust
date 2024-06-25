// Import anchor library
use anchor_lang::prelude::*;

// Here goes the public key that anchor will generate
declare_id!("Ak5FXCUH8ibcmp4aK2iBHo9FESbD4jDP1v5wAKtruFGN");

// Define program
#[program]

// Module to use account in
mod counter_program{

    // Import all of outside code
    use super::*;

    // Instruction 1 to create counter. It returns Ok as Result 
    pub fn create_counter(ctx: Context<Create>, first_number: u64) -> Result<()> {
        
        //Modify the number variable in counter account of the context create 
        ctx.accounts.counter.number = first_number;
        
        // Tell the authority of the account
        ctx.accounts.counter.authority = ctx.accounts.authority.key();

        // Message of this creation
        msg!("Creating counter with initial number {}");

        // Return
        Ok(())
    }

    pub fn delete_counter(_ctx: Context<Delete>) -> Result<()> {
        msg!("Counter deleted!");
        Ok(())
    }
}


// Macro devire to define context
#[derive(Accounts)]

// Struct of instructions to create
pub struct Create<'info> {

    // List of accounts

    // Macro account to indicate that the following are accounts
    // 8 bytes for discriminator plus 8 for the structure
    #[account(init, payer = authority, space = 8 + 8 + 32)]

    pub counter: Account<'info, Counter>,

    // Define Account as mutable 
    #[account(mut)]

    // Who signs the transaction
    pub authority: Signer<'info>,

    // Who creates the context. No macro needed.
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Delete<'info>{
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut, 
        constraint = counter.authority == counter.key(),
        close = authority
    )]
    pub counter: Account<'info, Counter>   
}

// Account counter
#[account]

// Counter Struct
pub struct Counter{
    // Define unsigned 64 (8 bytes) integer for the counter
    number: u64,

    // Authority added to the counter to be able to verify it
    // It is called on the function create_counter
    authority: Pubkey, // 32 bytes 
}





