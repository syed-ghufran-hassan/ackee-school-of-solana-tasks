//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // TODO: Implement deposit functionality

    let vault_authority = &mut ctx.accounts.user;

    let vault = &mut ctx.accounts.vault;

    let system_program = &mut ctx.accounts.system_program;

    let vault_authority_balance = **vault_authority.lamports.borrow();

    let is_locked = vault.locked;

    if vault_authority_balance < amount {

        return Err(VaultError::InsufficientBalance.into());
    }

    if is_locked {

        return Err(VaultError::VaultLocked.into());
    }

    let from = vault_authority.key();

    let to = vault.key();

    let deposit_ix = transfer(&from, &to, amount);

    invoke(&deposit_ix, &[vault_authority.to_account_info(), vault.to_account_info(), system_program.to_account_info()])?;

    emit!(DepositEvent{

        amount: amount,

        user: vault_authority.key(),

        vault: vault.key()

    });

    Ok(())
}
