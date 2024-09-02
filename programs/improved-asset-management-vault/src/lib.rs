use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("CkZiutQNW4qfdauyjbRPR5sCv5G7PV2arvsFGPez58iU");

#[program]
pub mod improved_asset_management_vault {
    use super::*;

    pub fn create_vault(ctx: Context<CreateVault>, name: String) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.manager = ctx.accounts.manager.key();
        vault.name = name;
        vault.total_value = 0;
        Ok(())
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let cpi_accounts = Transfer {
            from: ctx.accounts.investor_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.investor.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx: CpiContext<'_, '_, '_, '_, Transfer<'_>> = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        let investor_position = &mut ctx.accounts.investor_position;
        investor_position.amount += amount;
        
        let vault = &mut ctx.accounts.vault;
        vault.total_value += amount;

        emit!(DepositEvent {
            investor: ctx.accounts.investor.key(),
            amount,
            total_invested: investor_position.amount,
        });

        Ok(())
    }

    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);
        require!(ctx.accounts.investor.key() != ctx.accounts.vault.manager, VaultError::ManagerCannotWithdraw);
        
        let investor_position = &mut ctx.accounts.investor_position;
        require!(investor_position.amount >= amount, VaultError::InsufficientFunds);

        let vault_seeds = &[
            b"asset_vault".as_ref(),
            ctx.accounts.vault.name.as_bytes(),
            &[ctx.bumps.vault],
        ];
        let signer = &[&vault_seeds[..]];

        let transfer_to_investor = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.investor_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_investor,
            signer,
        );
        token::transfer(cpi_ctx, amount)?;

        investor_position.amount -= amount;
        let vault = &mut ctx.accounts.vault;
        vault.total_value -= amount;

        emit!(WithdrawEvent {
            investor: ctx.accounts.investor.key(),
            amount,
            remaining_balance: investor_position.amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateVault<'info> {
    #[account(
        init,
        payer = manager,
        space = 8 + 32 + 32 + 8 + 200,
        seeds = [b"asset_vault", name.as_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut, seeds = [b"asset_vault", vault.name.as_bytes()], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub investor: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = investor
    )]
    pub investor_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = investor,
        space = 8 + 32 + 32 + 8,
        seeds = [b"investor_position", vault.key().as_ref(), investor.key().as_ref()],
        bump
    )]
    pub investor_position: Account<'info, InvestorPosition>,
    #[account(
        init_if_needed,
        payer = investor,
        seeds = [b"vault_token", vault.key().as_ref(), token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut, seeds = [b"asset_vault", vault.name.as_bytes()], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub investor: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = investor
    )]
    pub investor_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"investor_position", vault.key().as_ref(), investor.key().as_ref()],
        bump
    )]
    pub investor_position: Account<'info, InvestorPosition>,
    #[account(
        mut,
        seeds = [b"vault_token", vault.key().as_ref(), token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub manager: Pubkey,
    pub name: String,
    pub total_value: u64,
}

#[account]
pub struct InvestorPosition {
    pub vault: Pubkey,
    pub investor: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Insufficient funds for withdrawal")]
    InsufficientFunds,
    #[msg("Vault manager cannot withdraw funds")]
    ManagerCannotWithdraw,
}

#[event]
pub struct DepositEvent {
    pub investor: Pubkey,
    pub amount: u64,
    pub total_invested: u64,
}

#[event]
pub struct WithdrawEvent {
    pub investor: Pubkey,
    pub amount: u64,
    pub remaining_balance: u64,
}