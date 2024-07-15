use {anchor_lang::prelude::*, crate::instructions::*};

// Presenting the created programs
mod collections;
mod instructions;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GUDM7qFaJqwAS2pnAZeeeydJm3BCvBBHEGXbgBoZFHd7");

#[program]
mod event_manager {
    use super::*;
    
    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        ticket_price: u64,
    ) -> Result<()> {
        instructions::create_event::handle(ctx, name, ticket_price)
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // // We must specify the space in order to initialize an account.
    // // First 8 bytes are default account discriminator,
    // // next 8 bytes come from NewAccount.data being type u64.
    // // (u64 = 64 bits unsigned integer = 8 bytes)
    // #[account(init, payer = signer, space = 8 + 8)]
    // pub new_account: Account<'info, NewAccount>,
    // #[account(mut)]
    // pub signer: Signer<'info>,
    // pub system_program: Program<'info, System>,
}

// #[account]
// pub struct NewAccount {
//     data: u64
// }