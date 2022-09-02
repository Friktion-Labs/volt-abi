use std::collections::{BTreeMap, BTreeSet};

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    ExtraVoltData, FriktionEpochInfo, PendingDeposit, PendingWithdrawal,
    PrincipalProtectionVaultV1, Round, VoltVault,
};

#[derive(Accounts, Clone)]
pub struct EntropyBaseAccountsWithoutBanks<'info> {
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    /// CHECK: skip, checked by macro
    #[account(address=extra_volt_data.entropy_program_id)]
    pub program: AccountInfo<'info>,
    /// CHECK: skip, checked by macro
    #[account( address=extra_volt_data.entropy_group)]
    pub group: AccountInfo<'info>,

    /// CHECK: skip, checked by macro
    #[account(address=extra_volt_data.entropy_cache)]
    pub cache: AccountInfo<'info>,

    /// CHECK: skip, checked by macro
    #[account(address=extra_volt_data.entropy_account)]
    pub account: AccountInfo<'info>,
}

impl<'info> EntropyBaseAccountsWithoutBanks<'info> {
    pub fn from_remaining_accounts(accounts: &mut &[AccountInfo<'info>]) -> Result<Self> {
        EntropyBaseAccountsWithoutBanks::try_accounts(
            &crate::id(),
            accounts,
            &vec![] as &[u8],
            &mut BTreeMap::<String, u8>::new(),
            &mut BTreeSet::<Pubkey>::new(),
        )
    }
}

#[derive(Accounts, Clone)]
pub struct PrincipalProtectionContextAccounts<'info> {
    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(
        seeds = [
          &volt_vault.key().to_bytes()[..],
          b"protectionVault"
      ],
      bump
    )]
    pub pp_vault: Box<Account<'info, PrincipalProtectionVaultV1>>,

    #[account(mut, address=pp_vault.get_deposit_tracking_account())]
    /// CHECK: checked by macro
    pub deposit_tracking_account: AccountInfo<'info>,

    #[account(mut, address=pp_vault.get_lending_shares_pool())]
    /// CHECK: checked by macro
    pub lending_shares_pool: Box<Account<'info, TokenAccount>>,

    #[account(mut, address=pp_vault.get_primary_lending_vault_pk())]
    /// CHECK: checked by macro
    pub lending_vault: AccountInfo<'info>,

    #[account(address=pp_vault.get_primary_lending_vault_program_id(), executable)]
    /// CHECK: checked by macro
    pub lending_vault_program: AccountInfo<'info>,
}

impl<'info> PrincipalProtectionContextAccounts<'info> {
    pub fn from_remaining_accounts(
        accounts: &mut &[AccountInfo<'info>],
    ) -> Result<PrincipalProtectionContextAccounts<'info>> {
        PrincipalProtectionContextAccounts::try_accounts(
            &crate::id(),
            accounts,
            &vec![] as &[u8],
            &mut BTreeMap::<String, u8>::new(),
            &mut BTreeSet::<Pubkey>::new(),
        )
    }
}

#[derive(Accounts)]
#[instruction(
    deposit_amount: u64,
)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub payer_authority: Signer<'info>,

    #[account(signer)]
    /// CHECK: skip, attributes may change depending on use case. validation happens in handler
    pub non_payer_authority: AccountInfo<'info>,

    /// CHECK: skip, attributes may change depending on use case. validation happens in handler
    pub authority_check: AccountInfo<'info>,

    #[account(mut, address=volt_vault.vault_mint)]
    pub vault_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(address=volt_vault.vault_authority)]
    /// CHECK: skip, checked by macro
    pub vault_authority: AccountInfo<'info>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    // main data struct. stores any persistent metadata about the volt and its strategy
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    #[account(address=extra_volt_data.whitelist)]
    /// CHECK: skip, checked by macro
    /// NOTE: this is a vector of pubkeys that can interact with the volt, NOT related to the whitelist MM token
    pub whitelist: AccountInfo<'info>,

    #[account(mut, address=volt_vault.deposit_pool)]
    pub deposit_pool: Box<Account<'info, TokenAccount>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.writer_token_pool)]
    pub writer_token_pool: AccountInfo<'info>,

    // no token::authority check here so that it can be either payer_authority or non_payer_authority
    #[account(mut)]
    // user controlled token account w/ mint == vault mint
    pub user_vault_tokens: Box<Account<'info, TokenAccount>>,

    // no token::authority check here so that it can be either payer_authority or non_payer_authority
    #[account(mut)]
    // user controlled token account w/ mint == underlying mint
    pub user_ul_tokens: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundInfo"],
        bump)]
    pub round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundVoltTokens"],
        bump)]
    pub round_volt_tokens: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundUnderlyingTokens"],
        bump)]
    pub round_underlying_tokens: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed,
        space=PendingDeposit::LEN + 8,
        seeds = [volt_vault.key().as_ref(), authority_check.key().as_ref() ,b"pendingDeposit"],
        bump,
        payer = payer_authority)]
    pub pending_deposit_info: Box<Account<'info, PendingDeposit>>,

    #[account(
        mut,
        seeds = [&volt_vault.key().to_bytes()[..], (volt_vault.round_number).to_le_bytes().as_ref() ,b"epochInfo"],
        bump,
    )]
    pub epoch_info: Box<Account<'info, FriktionEpochInfo>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
#[instruction(
    withdraw_amount: u64,
)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer_authority: Signer<'info>,

    #[account(signer)]
    /// CHECK: skip, attributes may change depending on use case. validation happens in handler,
    /// if is authority on token accounts (aka === authority_check), should be a signer
    pub non_payer_authority: AccountInfo<'info>,

    /// CHECK: skip, attributes may change depending on use case. validation happens in handler. hoowever,
    /// should be equal to 1 of authority or non_payer_authority
    pub authority_check: AccountInfo<'info>,

    #[account(mut)]
    pub vault_mint: Box<Account<'info, Mint>>,

    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
    )]
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.vault_authority)]
    pub vault_authority: AccountInfo<'info>,

    #[account(address=extra_volt_data.whitelist)]
    /// CHECK: skip, checked by macro
    pub whitelist: AccountInfo<'info>,

    #[account(mut, address=volt_vault.deposit_pool)]
    pub deposit_pool: Box<Account<'info, TokenAccount>>,

    // user token accounts
    #[account(mut)]
    pub underlying_token_destination: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub vault_token_source: Box<Account<'info, TokenAccount>>,

    // round accounts
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundInfo"],
        bump)]
    pub round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundUnderlyingTokens"],
        bump)]
    pub round_underlying_tokens: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed,
        space=PendingWithdrawal::LEN + 8,
        seeds = [volt_vault.key().as_ref(), authority_check.key().as_ref(), b"pendingWithdrawal"],
        bump,
        payer=payer_authority
    )]
    pub pending_withdrawal_info: Box<Account<'info, PendingWithdrawal>>,

    #[account(mut,
        seeds = [&volt_vault.key().to_bytes()[..], (volt_vault.round_number).to_le_bytes().as_ref() ,b"epochInfo"],
        bump,
    )]
    pub epoch_info: Box<Account<'info, FriktionEpochInfo>>,

    #[account(mut)]
    pub fee_acct: Box<Account<'info, TokenAccount>>,

    // system accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(
    deposit_amount: u64,
    do_sol_transfer: bool,
)]
pub struct DepositWithClaim<'info> {
    #[account(mut)]
    pub payer_authority: Signer<'info>,

    #[account(signer)]
    /// CHECK: skip, attributes may change depending on use case. validation happens in handler
    pub non_payer_authority: AccountInfo<'info>,

    #[account()]
    /// CHECK: skip, attributes may change depending on use case. validation happens in handler
    pub sol_transfer_authority: AccountInfo<'info>,

    /// CHECK: skip, attributes may change depending on use case. validation happens in handler
    #[account()]
    pub authority_check: AccountInfo<'info>,

    #[account(mut, address=volt_vault.vault_mint)]
    pub vault_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub volt_vault: Box<Account<'info, VoltVault>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.vault_authority)]
    pub vault_authority: AccountInfo<'info>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    // main data struct. stores any persistent metadata about the volt and its strategy
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    #[account(mut, address=volt_vault.deposit_pool)]
    pub deposit_pool: Box<Account<'info, TokenAccount>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.writer_token_pool)]
    pub writer_token_pool: AccountInfo<'info>,

    #[account(mut)]
    // user controlled token account w/ mint == vault mint
    pub vault_token_destination: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    // user controlled token account w/ mint == underlying mint
    pub underlying_token_source: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundInfo"],
        bump)]
    pub round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundUnderlyingTokens"],
        bump)]
    pub round_underlying_tokens: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed,
        space=PendingDeposit::LEN + 8,
        seeds = [volt_vault.key().as_ref(), authority_check.key().as_ref() ,b"pendingDeposit"],
        bump,
        payer = payer_authority)]
    pub pending_deposit_info: Box<Account<'info, PendingDeposit>>,

    /// CHECK: skip, checked by PDA, can't require an object because may not exist (if round number is 0)
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), pending_deposit_info.round_number.to_le_bytes().as_ref() , b"roundInfo"],
        bump)]
    pub pending_deposit_round_info: AccountInfo<'info>,

    /// CHECK: skip, checked by PDA, can't require an object because may not exist (if round number is 0)
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), pending_deposit_info.round_number.to_le_bytes().as_ref() , b"roundVoltTokens"],
        bump)]
    pub pending_deposit_round_volt_tokens: AccountInfo<'info>,

    /// CHECK: skip, checked by PDA, can't require an object because may not exist (if round number is 0)
    #[account(mut,
            seeds = [volt_vault.key().as_ref(), pending_deposit_info.round_number.to_le_bytes().as_ref() , b"roundUnderlyingTokens"],
            bump)]
    pub pending_deposit_round_underlying_tokens: AccountInfo<'info>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"epochInfo"],
        bump)]
    pub epoch_info: Box<Account<'info, FriktionEpochInfo>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(
    withdraw_amount: u64,
)]
pub struct WithdrawWithClaim<'info> {
    #[account(mut)]
    pub payer_authority: Signer<'info>,

    #[account(signer)]
    /// CHECK: skip, attributes may change depending on use case. validation happens in handler
    pub non_payer_authority: AccountInfo<'info>,

    /// CHECK: skip, attributes may change depending on use case. validation happens in handler. hoowever,
    /// should be equal to 1 of authority or non_payer_authority
    pub authority_check: AccountInfo<'info>,

    #[account(mut)]
    pub vault_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub volt_vault: Box<Account<'info, VoltVault>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.vault_authority)]
    pub vault_authority: AccountInfo<'info>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    // main data struct. stores any persistent metadata about the volt and its strategy
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    #[account(mut,address=volt_vault.deposit_pool)]
    pub deposit_pool: Box<Account<'info, TokenAccount>>,

    // user token accounts
    #[account(mut)]
    pub underlying_token_destination: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub vault_token_source: Box<Account<'info, TokenAccount>>,

    // round accounts
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundInfo"],
        bump)]
    pub round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundUnderlyingTokens"],
        bump)]
    pub round_underlying_tokens: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed,
        space=PendingWithdrawal::LEN + 8,
        seeds = [volt_vault.key().as_ref(), authority_check.key().as_ref(), b"pendingWithdrawal"],
        bump,
        payer = payer_authority)]
    pub pending_withdrawal_info: Box<Account<'info, PendingWithdrawal>>,

    /// CHECK: skip, checked by PDA. may be a non-existent account if there is no pendign withdrawal to process
    #[account(mut,
      seeds = [volt_vault.key().as_ref(), pending_withdrawal_info.round_number.to_le_bytes().as_ref() , b"roundInfo"],
      bump)]
    pub pending_withdrawal_round_info: AccountInfo<'info>,

    /// CHECK: skip, checked by PDA. may be a non-existent account if there is no pendign withdrawal to process
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), pending_withdrawal_info.round_number.to_le_bytes().as_ref() , b"roundUlPending"],
    bump)]
    pub pending_withdrawal_round_underlying_tokens_for_pws: AccountInfo<'info>,

    #[account(mut,
        seeds = [&volt_vault.key().to_bytes()[..], (volt_vault.round_number).to_le_bytes().as_ref() ,b"epochInfo"],
        bump,
    )]
    pub epoch_info: Box<Account<'info, FriktionEpochInfo>>,

    #[account(mut)]
    pub fee_acct: Box<Account<'info, TokenAccount>>,

    // system accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ClaimPendingDeposit<'info> {
    pub authority: Signer<'info>,

    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.vault_authority)]
    pub vault_authority: AccountInfo<'info>,

    #[account(mut, token::authority=authority.key())]
    // user controlled token account w/ mint == vault mint
    pub user_vault_tokens: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), pending_deposit_info.round_number.to_le_bytes().as_ref() , b"roundInfo"],
        bump)]
    pub pending_deposit_round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), pending_deposit_info.round_number.to_le_bytes().as_ref() , b"roundVoltTokens"],
        bump)]
    pub pending_deposit_round_volt_tokens: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), authority.key().as_ref(), b"pendingDeposit"],
        bump)]
    pub pending_deposit_info: Box<Account<'info, PendingDeposit>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimPendingWithdrawal<'info> {
    pub authority: Signer<'info>,

    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    /// CHECK: skip, checked by macro
    #[account(address=volt_vault.vault_authority)]
    pub vault_authority: AccountInfo<'info>,

    #[account(address=volt_vault.vault_mint)]
    pub vault_mint: Box<Account<'info, Mint>>,

    // user underlying token account
    #[account(mut, token::authority=authority.key())]
    pub underlying_token_destination: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), pending_withdrawal_info.round_number.to_le_bytes().as_ref() , b"roundInfo"],
        bump)]
    pub pending_withdrawal_round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), authority.key().as_ref(), b"pendingWithdrawal"],
        bump,
    )]
    pub pending_withdrawal_info: Box<Account<'info, PendingWithdrawal>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), (pending_withdrawal_info.round_number).to_le_bytes().as_ref() ,b"roundUlPending"],
        bump,
    )]
    pub round_underlying_tokens_for_pending_withdrawals: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelPendingDeposit<'info> {
    pub authority: Signer<'info>,

    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    #[account(address=volt_vault.vault_authority)]
    /// CHECK: skip, checked by macro
    pub vault_authority: AccountInfo<'info>,

    // user token accounts
    #[account(mut)]
    pub underlying_token_destination: Account<'info, TokenAccount>,

    // round accounts
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundInfo"],
        bump)]
    pub round_info: Box<Account<'info, Round>>,

    #[account(mut,
      seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundUnderlyingTokens"],
      bump)]
    pub round_underlying_tokens: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), authority.key().as_ref(), b"pendingDeposit"],
        bump)]
    pub pending_deposit_info: Box<Account<'info, PendingDeposit>>,

    #[account(mut,
        seeds = [&volt_vault.key().to_bytes()[..], (volt_vault.round_number).to_le_bytes().as_ref() ,b"epochInfo"],
        bump,
    )]
    pub epoch_info: Box<Account<'info, FriktionEpochInfo>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CancelPendingWithdrawal<'info> {
    pub authority: Signer<'info>,

    #[account(mut)]
    pub vault_mint: Account<'info, Mint>,

    pub volt_vault: Box<Account<'info, VoltVault>>,

    #[account(
        seeds = [
            &volt_vault.key().to_bytes()[..],
            b"extraVoltData"
        ],
        bump,
      )]
    pub extra_volt_data: Box<Account<'info, ExtraVoltData>>,

    #[account(address=volt_vault.vault_authority)]
    /// CHECK: skip, checked by macro
    pub vault_authority: AccountInfo<'info>,

    #[account(mut)]
    pub vault_token_destination: Account<'info, TokenAccount>,

    // round accounts
    #[account(mut,
        seeds = [volt_vault.key().as_ref(), volt_vault.round_number.to_le_bytes().as_ref() ,b"roundInfo"],
        bump)]
    pub round_info: Box<Account<'info, Round>>,

    #[account(mut,
        seeds = [volt_vault.key().as_ref(), authority.key().as_ref(), b"pendingWithdrawal"],
        bump)]
    pub pending_withdrawal_info: Box<Account<'info, PendingWithdrawal>>,

    #[account(mut,
        seeds = [&volt_vault.key().to_bytes()[..], (volt_vault.round_number).to_le_bytes().as_ref() ,b"epochInfo"],
        bump,
    )]
    pub epoch_info: Box<Account<'info, FriktionEpochInfo>>,

    // system accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
