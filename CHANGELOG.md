## [0.0.3] - 2022-07-30

### Refactors:
- modified field names of extra_volt_data

### Breaking:
- rename authority to payer_authority and dao_authority to non_payer_authority when both exist in a context
- remove mutable property from volt_vault account in Deposit, Withdraw
- chhange init_if_needed to mut for epoch_info account in Deposit
- move extra_volt_data account before vault_authority account in Withdraw
- remove mutable property from volt_vault and authority accounts in CancelPendingDeposit and CancelPendingWithdrawal
- remove vault_mint account from CancelPendingDeposit
- remove rent account from CancelPendingDeposit and CancelPendingWithdrawal
- renamed instruction claim_pending to claim_pending_deposit
- remove mutable property from authority accoun in ClaimPendingDeposit