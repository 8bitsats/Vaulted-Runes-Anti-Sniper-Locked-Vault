use anchor_lang::prelude::*;

declare_id!("E5gXFoHUpxHx97nyfvU6JGMWwC9f144HPYMnusrGysSB");

#[program]
pub mod vaultedrunes {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

declare_id!("YourVaultProgramID");

#[program]
pub mod alpha_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, lock_period: i64, max_cap: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        vault.authority = *ctx.accounts.authority.key;
        vault.lock_period = lock_period;
        vault.max_cap = max_cap;
        vault.total_deposited = 0;
        Ok(())
    }

    pub fn deposit_usdc(ctx: Context<DepositUSDC>, amount: u64) -> ProgramResult {
        let vault = &mut ctx.accounts.vault;
        let user_account = &mut ctx.accounts.user_account;
        let token_program = &ctx.accounts.token_program;
        
        // Transfer USDC from user to vault
        let cpi_accounts = token::Transfer {
            from: user_account.to_account_info(),
            to: vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        vault.total_deposited += amount;
        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> ProgramResult {
        // Claim logic based on average price and user's deposited amount
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 8)]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositUSDC<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    authority: Pubkey,
    lock_period: i64,
    max_cap: u64,
    total_deposited: u64,
}
