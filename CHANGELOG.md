## [0.0.3] - 2022-07-30

### Refactors:
- modified field names of extra_volt_data

### Breaking:
- rename authority to payer_authority and dao_authority to non_payer_authority when both exist in a context
- remove init_if_needed from epoch_info in Deposit
- move extra_volt_data before vault_authority in Withdraw context
- remove mutable property from volt_vault account in CancelPendingDeposit and CancelPendingWithdrawal
- remove rent account from CancelPendingDeposit and CancelPendingWithdrawal
- renamed instruction claim_pending to claim_pending_deposit