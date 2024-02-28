use std::mem;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("DB8KUsBQczCSyYedg5f5wPEz88J1V4YbN3BGAzFFCHvH");

#[program]
pub mod token_locked {
    use super::*;

    pub fn init_account(ctx: Context<Deposit>, amount: u64, unlock_time: i64) -> Result<()> {
        let locker = &mut ctx.accounts.lock_account;
        let fee_percentage = 5; // Representing 0.4%
        let fee_divisor = 1000; // 100% divided by 0.4%
        let fee_amount: u64 = (amount * fee_percentage) / fee_divisor;

        // Calculate remaining amount after deducting the fee
        let remaining: u64 = amount - fee_amount;
        let transfer_instruction = Transfer {
            from: ctx.accounts.signer_ata.to_account_info(),
            to: ctx.accounts.fee_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        let transfer_amount_instruction = Transfer {
            from: ctx.accounts.signer_ata.to_account_info(),
            to: ctx.accounts.lock_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi2_program = ctx.accounts.token_program.to_account_info();
        let cpi2_context = CpiContext::new(cpi2_program, transfer_amount_instruction);
        locker.locked_amount = remaining;
        locker.unlock_time = unlock_time;
        locker.mint = ctx.accounts.mint.key();
        locker.owner = ctx.accounts.signer.key();
        anchor_spl::token::transfer(cpi_ctx, fee_amount)?;
        anchor_spl::token::transfer(cpi2_context, remaining)?;
        Ok(())
    }
    pub fn increase_amount(ctx: Context<IncreaseDeposit>, amount: u64, unlock_time: i64) -> Result<()> {
        let locker = &mut ctx.accounts.lock_account;
        let fee_percentage = 5; // Representing 0.4%
        let fee_divisor = 1000; // 100% divided by 0.4%
        let fee_amount: u64 = (amount * fee_percentage) / fee_divisor;
        let time = Clock::get()?;
        require!(
            locker.unlock_time <= time.unix_timestamp * 1000,
            LockErrors::WrongTimeToUnlock
        );
        // Calculate remaining amount after deducting the fee
        let remaining: u64 = amount - fee_amount;
        let transfer_instruction = Transfer {
            from: ctx.accounts.signer_ata.to_account_info(),
            to: ctx.accounts.fee_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

        let transfer_amount_instruction = Transfer {
            from: ctx.accounts.signer_ata.to_account_info(),
            to: ctx.accounts.lock_ata.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        let cpi2_program = ctx.accounts.token_program.to_account_info();
        let cpi2_context = CpiContext::new(cpi2_program, transfer_amount_instruction);
        locker.locked_amount += remaining;
        locker.unlock_time = unlock_time;

        anchor_spl::token::transfer(cpi_ctx, fee_amount)?;
        anchor_spl::token::transfer(cpi2_context, remaining)?;
        Ok(())
    }
    pub fn close_and_withdraw(ctx: Context<CloseAndWithdraw>) -> Result<()> {
        let time = Clock::get()?;
        let locker = &mut ctx.accounts.lock_account;
        require!(
            locker.unlock_time <= time.unix_timestamp * 1000,
            LockErrors::WrongTimeToUnlock
        );
        let binding = ctx.accounts.signer.key();
        let mint_adr= ctx.accounts.mint.key(); 
        let seeds = &["guac_lock".as_bytes(), binding.as_ref(),mint_adr.as_ref()];
        let (_derived_address, bump) = Pubkey::find_program_address(seeds, &ctx.program_id);
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.lock_ata.to_account_info(),
                    to: ctx.accounts.signer_ata.to_account_info(),
                    authority: locker.to_account_info(),
                },
                &[&["guac_lock".as_bytes(), binding.as_ref(),mint_adr.as_ref(), &[bump]]],
            ),
            locker.locked_amount,
        )?;

        Ok(())
    }
}
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(init, payer = signer, space = mem::size_of::<LockerAccount>()+8, seeds = ["guac_lock".as_bytes(), signer.key().as_ref(), mint.key().as_ref()], bump)]
    pub lock_account: Account<'info, LockerAccount>,
    #[account(mut)]
    pub lock_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct LockerAccount {
    pub locked_amount: u64,
    pub unlock_time: i64,
    pub owner: Pubkey,
    pub mint: Pubkey,
}
#[derive(Accounts)]
pub struct CloseAndWithdraw<'info> {
    /// CHECK: closing voting account
    #[account(mut,close=signer, seeds = ["guac_lock".as_bytes(), signer.key().as_ref(), mint.key().as_ref()], bump)]
    pub lock_account: Account<'info, LockerAccount>,
    #[account(mut)]
    pub lock_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct IncreaseDeposit<'info> {
    #[account(mut, seeds = ["guac_lock".as_bytes(), signer.key().as_ref(), mint.key().as_ref()], bump)]
    pub lock_account: Account<'info, LockerAccount>,
    #[account(mut)]
    pub lock_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub fee_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum LockErrors {
    #[msg("Amount is less than minimeum")]
    AmountIsLessThanMin,
    #[msg("Too early to withdraw")]
    WrongTimeToUnlock,
}
