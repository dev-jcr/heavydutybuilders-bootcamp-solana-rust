use anchor_lang::prelude::*;

// Define structure event
#[account]
#[derive(InitSpace)] // This macro calculates the required space for the account
pub struct Event {

    // It will be stored as a PDA so can be the only one to modify    

    // Three main requirements
    #[max_len(40)]
    pub name: String,      // Name
    pub ticket_price: u64, // Ticket Price
    pub active: bool,      // Event status

    // Two needed accounts being usually used
    pub authority: Pubkey, // Organizer
    pub accepted_mint: Pubkey // Accepted coin

    // Bumps of all of the PDA's
    pub event_bump: u8, // Of our account
    pub event_mint_bump: u8, // of our token
    pub treasury_vault_bump: u8, // treasury
    pub gain_vault_bump: u8, // vault

}

// Extra seed as static string associated to our type of event with an implementation 'impl'

 impl Event {
    pub const SEED_EVENT: &'static str = "event";
    pub const SEED_EVENT_MINT: &'static str = "event_mint";
    pub const SEED_TREASURY_VAULT: &'static str = "treasury_vault";
    pub const SEED_GAIN_VAULT: &'static str = "gain_vault";

 }

// To expose the structure the file mod.rs will be created

