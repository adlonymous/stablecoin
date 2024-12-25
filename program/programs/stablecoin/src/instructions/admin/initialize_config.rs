use anchor_lang::prelude::*;
use crate::{Config, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT, MINT_DECIMALS, LIQUIDATION_THRESHOLD, LIQUIDATION_BONUS, MIN_HEALTH_FACTOR};
use anchor_spl::token_interface::{Mint, Token2022};

#[derive(Account)]
pub struct InitializeConfig {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account : Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_initialize_config(
    ctx: Context<InitializeConfig>,
    
) -> Result<()> {
    *ctx.accounts.config_account = Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATION_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        minimum_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.accounts.config_account.bump,
        bump_mint: ctx.bumps.config_account,
    };

    Ok(())
}