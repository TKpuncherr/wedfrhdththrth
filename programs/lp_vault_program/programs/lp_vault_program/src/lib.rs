use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("9LpVaUltPrg111111111111111111111111111111111");

#[program]
pub mod lp_vault_program {
    use super::*;

    pub fn withdraw_lp(ctx: Context<WithdrawLp>, bump: u8, amount: u64) -> Result<()> {
        let vault_pda = &ctx.accounts.vault_pda;
        let lp_mint = &ctx.accounts.lp_mint;

        let seeds: &[&[u8]] = &[
            b"lp_vault",
            lp_mint.key().as_ref(),
            &[bump],
        ];
        let signer_seeds: &[&[&[u8]]] = &[seeds];

        let vault_ata = &ctx.accounts.vault_lp_ata;
        let user_ata = &ctx.accounts.user_lp_ata;

        let amount_to_transfer = if amount == 0 {
            vault_ata.amount
        } else {
            amount
        };

        require!(amount_to_transfer > 0, LpVaultError::NothingToWithdraw);

        let cpi_accounts = Transfer {
            from: vault_ata.to_account_info(),
            to: user_ata.to_account_info(),
            authority: vault_pda.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount_to_transfer)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct WithdrawLp<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = user_lp_ata.owner == user.key(),
        constraint = user_lp_ata.mint == lp_mint.key(),
    )]
    pub user_lp_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"lp_vault", lp_mint.key().as_ref()],
        bump,
    )]
    pub vault_pda: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault_lp_ata.owner == vault_pda.key(),
        constraint = vault_lp_ata.mint == lp_mint.key(),
    )]
    pub vault_lp_ata: Account<'info, TokenAccount>,

    pub lp_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum LpVaultError {
    #[msg("Nothing to withdraw from vault")]
    NothingToWithdraw,
}
