use anchor_lang::prelude::*;

use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;
use error::*;
mod error;

declare_id!("5F9uizLngFBHGjPJGz3WqCpoic9yoa1zRGpHTUnWhALt");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, minimum_health_factor: u64) -> Result<()> {
        process_update_config(ctx, minimum_health_factor)
    }
}

