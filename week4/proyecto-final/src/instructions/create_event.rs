// Context create_event.rs
use {crate::collections::Event, anchor_lang::prelude*, anchor_spl::token*};

// First add to the Cargo.toml at [dependencies]:
//anchor-spl = "0.29.0"

#[derive(Accounts)]
pub struct CreateEvent<'info> {

    // Create account with macro
    #[account(

        // Add seeds since it is a PDA
        seeds = [
            Event::SEED_EVENT.as_ref(), // "event" saved on impl of event.rs
            authority.key().as_ref(), // event authority to know who created the event and has the power to act within it
        ],
        bump,
        init, // Init account
        payer = authority, // Who pays
        space = 8 + Event::INIT_SPACE, // Space required for the account adding the calculated space to the required 8
    )]
    
    // Event account with structure Event
    pub event: Account<'info, Event>,

    // Accepted mint account
    pub accepted_mint: Account<'info, Mint>,

    // Event account
    #[account(
        init,
        // PDA seeds
        seeds = [
            // Associate event with it
            Event::SEED_EVENT_MINT.as_ref(), // "event_mint"
            event.key().as_ref() // associate to a specific event
        ],
        bump,
        payer = authority,
        mint::decimals = 0, // No decimals for the mint
        mint::authority = event, // Authority to mint. In this case, trough program 'event'

    )]
    
    pub event_mint: Account<'info, Mint>,


    // Token accounts: Treasury and Gain Vaults

    // Treasury Vault

    #[account(
        init,
        payer = authority,
        seeds = [
            Event::SEED_TREASURY_VAULT.as_ref(), // "treasury_vault"
            event.key().as_ref() // associate to a specific event
        ],
        bump,
        token::mint = accepted_mint, // accepted mint to be store 
        token::authority = event, // authority that uses this token account
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    // Gain  Vault

    #[account(
        init,
        payer = authority,
        seeds = [
            Event::SEED_GAIN_VAULT.as_ref(), // "gain_vault"
            event.key().as_ref() // associate to a specific event
        ],
        bump,
        token::mint = accepted_mint, // accepted mint to be store 
        token::authority = event, // authority that uses this token account
    )]
    pub gain_vault: Account<'info, TokenAccount>,

    // Programs
    #[account(mut)]
    pub authority: Signer<'info>, // Event authority program
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>, // Token program
    pub system_program: Program<'info, System>, // System program for the accounts being created

}

// Additional to context: Action that will be done when this instruction is called
pub fn handle(ctx: Context<CreateEvent>, name: String, ticket_price: u64) -> Result<()> {
    // Update event account data
    ctx.accounts.event.name = name;
    ctx.accounts.event.ticket_price = ticket_price;
    ctx.accounts.event.active = true;

    // Authority and mint
    ctx.accounts.event.authority = ctx.accounts.authority.key(); // Choose same authority of context
    ctx.accounts.event.accepted_mint = ctx.accounts.accepted_mint.key(); //Choose to accept the same mint of the event

    // Bumps generated with the PDA's
    ctx.accounts.event.event_bump = ctx.bumps.event;
    ctx.accounts.event.event_mint_bump = ctx.bumps.event_mint;
    ctx.accounts.event.treasury_vault_bump = ctx.bumps.treasury_vault
    ctx.accounts.event.gain_vault_bump = ctx.bumps.gain.vault;
    Ok(())
}