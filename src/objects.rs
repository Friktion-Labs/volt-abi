use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Whitelist {
    pub admin: Pubkey,

    /// The storage for information on reserves in the market
    pub addresses: Vec<Pubkey>,
}

#[account]
#[derive(Default)]
/**
 * User-specific PDA. Tracks information about their pending deposits.
 *  NOTES:
 *  1. There may only be one pending deposit (across all rounds) at any point in time
 *  2. However, pending deposits will accumulate if made in same round.
 *  3. Pending deposits from previous rounds may be claimed with the instruction "claim_pending"
 */
pub struct PendingDeposit {
    // unnecessary variable. indicates whether account exists
    pub initialized: bool,

    // round number this pending deposit is for.
    // #NOTE: round_number == 0 implies "no existing pending deposit"
    pub round_number: u64,

    // total amount that is or was pending to be deposited.
    // this number is used to calculate the # of volt tokens user should expect to receive after calling claim_pending.
    // this is incremented with new deposits, and decremented after calling claim_pending
    pub num_underlying_deposited: u64,
}

impl PendingDeposit {
    pub const LEN: usize = 17;
}

#[account]
#[derive(Default)]
/**
 * User-specific PDA. Tracks information about their pending withdrawals.
 *  NOTES:
 *  1. There may only be one pending withdrawal (across all rounds) at any point in time
 *  2. However, pending withdrawals will accumulate if made in same round.
 *  3. Pending withdrawals from previous rounds may be claimed with the instruction "claim_pending_withdrawal"
 */
pub struct PendingWithdrawal {
    // unnecessary variable. indicates whether account exists
    pub initialized: bool,

    // round number this pending withdrawal is for.
    // #NOTE: round_number == 0 implies "no existing pending withdrawal"
    pub round_number: u64,

    // total amount that is or was pending to be deposited.
    // this number is used to calculate the # of volt tokens user should expect to receive after calling claim_pending.
    // this is incremented with new deposits, and decremented after calling claim_pending
    pub num_volt_redeemed: u64,
}

impl PendingWithdrawal {
    pub const LEN: usize = 17;
}

#[account]
#[derive(Default)]
pub struct UlOpenOrdersMetadata {
    initialized: bool, // 1
}

impl UlOpenOrdersMetadata {
    pub const LEN: usize = 1;
}

#[account]
#[derive(Default, PartialEq, Debug)]
pub struct EntropyRound {
    pub instant_deposits_native: u64,
    pub prev_entropy_account_deposits: u64,
    pub initial_equity: f64,             //
    pub new_equity_post_deposit: f64,    // 16
    pub deposit_amt: f64,                // 24
    pub withdraw_comp_from_deposit: u64, // 32
    pub net_deposits: f64,               // 40
    pub deposit_amt_native: u64,         // 48
    pub withdraw_amt_native: u64,        // 56
    pub total_volt_supply: u64,
    pub oracle_price: f64,

    pub acct_equity_start: f64,
    pub acct_equity_before_next_rebalance: f64,
    pub pnl_quote: f64,
    pub performance_fees_quote: f64,

    pub temp1: Pubkey,        // 368
    pub temp2: Pubkey,        // 400
    pub temp3: Pubkey,        // 432
    pub extra_key_11: Pubkey, // 464
    pub extra_key_12: Pubkey, // 496

    pub unused_uint_four: u64, // 504
    pub unused_uint_five: u64, // 512
    pub unused_uint_six: u64,  // 520
    pub unused_uint_12: u64,   // 568

    pub unused_float1: f64, // 504
    pub unused_float2: f64, // 512
    pub unused_float3: f64, // 520
    pub unused_float4: f64, // 568

    pub unused_bool_one: bool,   // 569
    pub unused_bool_two: bool,   // 570
    pub unused_bool_three: bool, // 571
    pub unused_bool_four: bool,
}

impl EntropyRound {
    pub const LEN: usize = 348;
}

#[account]
#[derive(Default)]
/**
 * Epoch-specific PDA. Stores all information specific to that epoch.
 * New rounds are initialized in start_round.
 * Modified in deposit, withdraw, claim_pending, claim_pending_withdrawal
 */
pub struct Round {
    // numerical ranking of this round. round numbers start at 1, and are incremented
    // following each successful call of start_round
    pub number: u64,

    // total # of underlying from pending deposits created during this round.
    // NOTE: this has to be stored in the Round account in order to calculate correct
    // proportion of vault tokens for each user in claim_pending
    pub underlying_from_pending_deposits: u64,

    // total # of volt tokens from pending withdrawals created during this round.
    // NOTE: this has to be stored in the Round account in order to calculate correct
    // proportion of underlying tokens for each user in claim_pending_withdrawal
    pub volt_tokens_from_pending_withdrawals: u64,

    pub underlying_pre_enter: u64,

    pub underlying_post_settle: u64,

    pub premium_farmed: u64,
    // pub unused_space: [u64; 7],
}

impl Round {
    pub const LEN: usize = 48;
}

#[account]
#[derive(Default)]
pub struct FriktionEpochInfo {
    pub vault_token_price: f64,      // 8
    pub pct_pnl: f64,                // 16
    pub number: u64,                 // 24
    pub underlying_pre_enter: u64,   // 32
    pub underlying_post_settle: u64, // 40
    pub volt_token_supply: u64,      // 48
    pub pnl: i64,                    // 56

    pub performance_fees: u64, // 64
    pub withdrawal_fees: u64,  // 72

    pub pending_deposits: u64, // 80

    pub pending_withdrawals_volt_tokens: u64, // 88
    pub pending_withdrawals: u64,             // 96
    // in volt tokens
    pub canceled_withdrawals: u64, // 104
    pub canceled_deposits: u64,    // 112
    pub total_withdrawals: u64,    // 120
    pub total_deposits: u64,       // 128
    pub instant_deposits: u64,     // 136
    pub instant_withdrawals: u64,  // 144
    pub dao_deposits: u64,         // 152
    pub minted_options: u64,       // 160

    pub enter_num_times_called: u64,        // 168
    pub swap_premium_num_times_called: u64, // 176

    pub option_key: Pubkey, // 208

    pub extra_key_four: Pubkey, // 240
    pub extra_key_5: Pubkey,    // 272
    pub extra_key_6: Pubkey,    // 304
    pub extra_key_7: Pubkey,    // 336
    pub extra_key_8: Pubkey,    // 368
    pub extra_key_9: Pubkey,    // 400
    pub extra_key_10: Pubkey,   // 432
    pub extra_key_11: Pubkey,   // 464
    pub extra_key_12: Pubkey,   // 496

    pub unused_uint_four: u64, // 504
    pub unused_uint_five: u64, // 512
    pub unused_uint_six: u64,  // 520
    pub unused_uint_7: u64,    // 528
    pub unused_uint_8: u64,    // 536
    pub unused_uint_9: u64,    // 544
    pub unused_uint_10: u64,   // 552
    pub unused_uint_11: u64,   // 560
    pub unused_uint_12: u64,   // 568

    pub unused_bool_one: bool,   // 569
    pub unused_bool_two: bool,   // 570
    pub unused_bool_three: bool, // 571
    pub unused_bool_four: bool,  // 572
    pub unused_bool_five: bool,  // 573
    pub unused_bool_six: bool,   // 574
}

impl FriktionEpochInfo {
    pub const LEN: usize = 574;
}

#[account]
#[derive(Default, Debug)]
pub struct EntropyMetadata {
    // generic across all entropy vaults
    pub target_hedge_ratio: f64,

    pub rebalancing_lenience: f64,

    // basis volt stuff
    pub required_basis_from_oracle: f64,

    pub extra_key_3: Pubkey,  // 368
    pub extra_key_4: Pubkey,  // 400
    pub extra_key_5: Pubkey,  // 432
    pub extra_key_6: Pubkey,  // 464
    pub extra_key_7: Pubkey,  // 496
    pub extra_key_8: Pubkey,  // 368
    pub extra_key_9: Pubkey,  // 400
    pub extra_key_10: Pubkey, // 432
    pub extra_key_11: Pubkey, // 464
    pub extra_key_12: Pubkey, // 496

    pub unused_uint_four: u64, // 504
    pub unused_uint_five: u64, // 512
    pub unused_uint_six: u64,  // 520
    pub unused_uint_12: u64,   // 568
    pub unused_uint_123: u64,  // 504
    pub unused_uint_456: u64,  // 512
    pub unused_uint_789: u64,  // 520
    pub unused_uint_102: u64,  // 568

    pub unused_float1: f64,  // 504
    pub unused_float2: f64,  // 512
    pub unused_float3: f64,  // 520
    pub unused_float4: f64,  // 568
    pub unused_float5: f64,  // 504
    pub unused_float6: f64,  // 512
    pub unused_float7: f64,  // 520
    pub unused_float8: f64,  // 568
    pub unused_float9: f64,  // 504
    pub unused_float10: f64, // 512
    pub unused_float11: f64, // 520
    pub unused_float12: f64, // 568

    pub unused_bool_one: bool,   // 569
    pub unused_bool_two: bool,   // 570
    pub unused_bool_three: bool, // 571
    pub unused_bool_four: bool,
    pub unused_bool_five: bool,  // 569
    pub unused_bool_six: bool,   // 570
    pub unused_bool_seven: bool, // 571
    pub unused_bool_eight: bool,
    pub unused_bool_nine: bool, // 571
    pub unused_bool_ten: bool,

    pub vault_name: String,
}

impl EntropyMetadata {
    pub const LEN: usize = 512 + 30;
}

#[account]
#[derive(Default, Copy, Debug)]
pub struct ExtraVoltData {
    pub is_whitelisted: bool, // 1

    pub whitelist: Pubkey, // 33

    pub is_for_dao: bool, // 34

    pub dao_program_id: Pubkey, // 66

    // spl mint of deposit token
    pub deposit_mint: Pubkey, // 98

    // target leverage amount (as ratio) for the position (for each rebalance)
    pub target_leverage: f64, // 106

    // defines width of interval collateralization ratio at end of round must lie within
    pub target_leverage_lenience: f64, // 114

    // leverage threshold for calling exit_early instructoin
    pub exit_early_ratio: f64, // 130

    // is this trading on mango or entropy (or 01 :P)?
    pub entropy_program_id: Pubkey, // 194

    // group the protocol trades on
    pub entropy_group: Pubkey, // 226 186

    // account the program initializes to trade with
    pub entropy_account: Pubkey, // 258 218

    // pubkey of perp market to trade
    pub power_perp_market: Pubkey, // 322

    // true after settle_deposits was called successfully for current round
    pub have_resolved_deposits: bool, // 355

    // true after obtained target collateralization, ready to end round
    pub done_rebalancing: bool, // 356

    pub dao_authority: Pubkey,    // 388
    pub serum_program_id: Pubkey, // 420
    pub entropy_cache: Pubkey,    // 348 (actual)
    /// pubkey of perp market to hedge
    pub spot_perp_market: Pubkey, // 354 (380)
    pub extra_key_7: Pubkey,      // 516  412
    pub extra_key_8: Pubkey,      // 548 444
    pub extra_key_9: Pubkey,      // 580
    pub extra_key_10: Pubkey,     // 612
    pub extra_key_11: Pubkey,     // 644
    pub extra_key_12: Pubkey,
    pub extra_key_13: Pubkey,
    pub extra_key_14: Pubkey, // 636

    pub net_withdrawals: u64,      // 644
    pub max_quote_pos_change: u64, // 652
    // defines width of dollar delta range hedge must lie within
    pub target_hedge_lenience: f64, // 122
    pub unused_uint_four: u64,      // 676
    pub unused_uint_five: u64,      // 684
    pub unused_uint_six: u64,       // 692
    pub unused_uint_7: u64,         // 700
    pub unused_uint_8: u64,         // 708
    pub unused_uint_9: u64,         // 716
    pub unused_uint_10: u64,        // 724
    pub unused_uint_11: u64,        //
    pub unused_uint_12: u64,        // 732

    pub turn_off_deposits_and_withdrawals: bool, //
    pub rebalance_is_ready: bool,                //
    pub unused_bool1234: bool,                   //
    pub done_rebalancing_power_perp: bool,       //
    pub is_hedging_on: bool,                     //
    pub have_taken_performance_fees: bool,       // 738
}

impl ExtraVoltData {
    pub const LEN: usize = 738;
}

#[account]
#[derive(Default, Copy, Debug)]
pub struct VoltVault {
    // permissioned instructions
    pub admin_key: Pubkey, // 32

    pub seed: Pubkey, // 64

    ///////// time windows ///////////

    // length of withdrawal window in seconds
    pub transfer_window: u64, // 72

    // time at which withdrawals began
    pub start_transfer_time: u64, // 80

    // minimum time at which withdrawals end
    pub end_transfer_time: u64, // 88

    ///////// rebalance pipeline state ///////////

    // has vault been initialized?
    pub initialized: bool, // 89

    // has the current/previous options position been settled?
    // settlement is defined as having all AUM of this volt stored in one asset (underlying)
    pub curr_option_was_settled: bool, // 90

    // do we have to swap premium to underlying before entering position?
    pub must_swap_premium_to_underlying: bool, // 91

    // has the next option to roll into been set?
    pub next_option_was_set: bool, // 92

    // has set_next_option been called successfully a single time yet?
    pub first_ever_option_was_set: bool, // 93

    // should deposit/withdraw still be validly callable?
    pub instant_transfers_enabled: bool, // 94

    // has rebalance prepare been called this cycle?
    pub prepare_is_finished: bool, // 95

    // are we out of options to sell in the rebalance enter stage of this cycle?
    pub enter_is_finished: bool, // 96

    // has the current round started?
    pub round_has_started: bool, // 97

    // round number of roll. increments post-settlement (maybe set next option)
    pub round_number: u64, // 105

    ////// Rewards Tracking //////

    // total amount in vault when set next option was last called
    pub total_underlying_pre_enter: u64, // 113

    // total amount in vault immediately following full settlement
    pub total_underlying_post_settle: u64, // 121

    // total number of volt tokens in supply post settlement
    // used for calculating share of pnl
    pub total_volt_tokens_post_settle: u64, // 129

    ///////// accounts to save here so others can read. additionally for account_validators.rs ///////////
    pub vault_authority: Pubkey, // 161

    // pools
    pub deposit_pool: Pubkey, // 193

    pub premium_pool: Pubkey, // 225

    pub option_pool: Pubkey, // 257

    pub writer_token_pool: Pubkey, // 289

    // mints
    pub vault_mint: Pubkey, // 321

    pub underlying_asset_mint: Pubkey, // 353

    pub quote_asset_mint: Pubkey, // 385

    pub option_mint: Pubkey, // 417

    pub writer_token_mint: Pubkey, // 449

    pub option_market: Pubkey, // 481

    ///////// vault strategy params ///////////

    // integer describing the vault strategy
    pub vault_type: u64, // 489

    /// The amount of the **underlying asset** that derives a single option
    pub underlying_amount_per_contract: u64, // 497

    // strike * underlying per contract
    pub quote_amount_per_contract: u64, // 505

    /// The Unix timestamp at which the contracts in this market expire
    pub expiration_unix_timestamp: i64, // 513

    // exact expiry length the target option should have, except for the initial option
    pub expiration_interval: u64, // 521

    // option is assumed to be OTM. this limits how high the strike can be relative to current underlying price.
    // i.e, new strike must be less than or equal to (underlying price * upper_bound_otm_strike_factor)
    // NOTE: value should be given from client as (desired factor) * 10, so need to divide by 10 to get actual normalization
    pub upper_bound_otm_strike_factor: u64, // 529

    /// A flag to set and use to when running a memcmp query.
    /// This will be set when Serum markets are closed and expiration is validated
    pub have_taken_withdrawal_fees: bool, // 530

    ///////// serum ///////////
    pub serum_spot_market: Pubkey, // 562

    // bump for program address of open orders account (serum)
    pub open_orders_bump: u8, // 563

    // bump for serum (unknown)
    pub open_orders_init_bump: u8, // 564

    // bump for open orders for underlying serum market
    pub ul_open_orders_bump: u8, // 565

    // open orders account for underlying spot market
    pub ul_open_orders: Pubkey, // 597

    // was the ul open orders acct already initialized?
    pub ul_open_orders_initialized: bool, // 598

    // bump for vault authority, used to generate signer seeds for CPI calls
    pub bump_authority: u8, // 599

    // order size in # contracts to sell options
    pub serum_order_size_options: u64, // 607

    // order size in # contracts to sell options
    pub individual_capacity: u64, // 615

    // type of order to submit (limit, postonly, IOC)
    pub serum_order_type: u64, // 623

    // unknown usage, set to 65535 works
    pub serum_limit: u16, // 625

    // specifies what serum should do if the order is matched w/ an order from the same account
    pub serum_self_trade_behavior: u16, // 627

    // order ID to use when sending a serum order. should be incremented following each succesful order
    pub serum_client_order_id: u64, // 635
    // pub whitelist: Pubkey,
    // pub unused_space:
    // pub unused_space: [u64; 30],
    pub whitelist_token_mint: Pubkey, // 667

    pub permissioned_market_premium_mint: Pubkey, // 699
    pub permissioned_market_premium_pool: Pubkey, // 731
    // pub extra_key_two:   ,
    pub capacity: u64, // 739
                       // pub unused_uint_two: u64,
                       // pub extra_key_three: Pubkey,
                       // pub extra_key_four: Pubkey,

                       // pub unused_uint_one: u64,
                       // pub unused_uint_two: u64,
                       // pub unused_uint_three: u64,
                       // pub unused_uint_four: u64,
                       // pub unused_uint_five: u64,
                       // pub unused_uint_six: u64,
}

impl VoltVault {
    pub const LEN: usize = 739;
}
