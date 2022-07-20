use anchor_lang::prelude::*;
declare_id!("VoLT1mJz1sbnxwq5Fv2SXjdVDgPXrb9tJyC8WpMDkSp");

pub mod contexts;
pub mod error;
pub mod objects;

pub use contexts::*;
pub use error::*;
pub use objects::*;

#[program]
mod volt_abi {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(clippy::too_many_arguments)]

    use super::*;

    pub(crate) fn deposit(ctx: Context<Deposit>, deposit_amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn deposit_with_claim(
        ctx: Context<DepositWithClaim>,
        amount: u64,
        do_transfer: bool,
    ) -> Result<()> {
        Ok(())
    }

    pub(crate) fn withdraw_with_claim(ctx: Context<WithdrawWithClaim>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub(crate) fn claim_pending(ctx: Context<ClaimPendingDeposit>) -> Result<()> {
        Ok(())
    }
    pub(crate) fn claim_pending_withdrawal(ctx: Context<ClaimPendingWithdrawal>) -> Result<()> {
        Ok(())
    }

    pub(crate) fn cancel_pending_deposit(ctx: Context<CancelPendingDeposit>) -> Result<()> {
        Ok(())
    }
    pub(crate) fn cancel_pending_withdrawal(ctx: Context<CancelPendingWithdrawal>) -> Result<()> {
        Ok(())
    }
    // ========== TRADING ==========
}
