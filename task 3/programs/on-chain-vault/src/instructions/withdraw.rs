//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(mut,
    has_one = vault_authority)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // TODO: Implement withdraw functionality
    
    let vault = &mut ctx.accounts.vault;

    let vault_authority = &mut ctx.accounts.vault_authority;

    //let system_program = &mut ctx.accounts.system_program;

    let is_locked = vault.locked;

    if is_locked {

        return Err(VaultError::VaultLocked.into());
    }

    let vault_balance = vault.get_lamports();

    if amount > vault_balance {

        return Err(VaultError::InsufficientBalance.into());
    }

    **vault.to_account_info().try_borrow_mut_lamports()? -= amount;

    **vault_authority.to_account_info().try_borrow_mut_lamports()? += amount;

    // let from = vault.key();

    // let to = vault.vault_authority;

    // let withdraw_ix = transfer(&from, &to, amount);

    // let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault", vault_authority.key().as_ref()], ctx.program_id);

    // invoke_signed(&withdraw_ix, &[vault.to_account_info(), vault_authority.to_account_info(), system_program.to_account_info()],

    // &[&[b"vault", vault_authority.key().as_ref(), &[vault_bump]]])?;

    emit!(WithdrawEvent{

        amount: amount,

        vault: vault.key(),

        vault_authority: vault.vault_authority
    });

    Ok(())
}
