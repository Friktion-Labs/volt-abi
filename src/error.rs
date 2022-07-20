use anchor_lang::error_code;
use solana_program::msg;

#[error_code]
pub enum ErrorCode {
    #[msg("Expiration must be in the future")]
    ExpirationIsInThePast,
    #[msg("Same quote and underlying asset, cannot create market")]
    QuoteAndUnderlyingAssetMustDiffer,
    #[msg("Quote amount and underlying amount per contract must be > 0")]
    QuoteOrUnderlyingAmountCannotBe0,
    #[msg("OptionMarket must be the mint authority")]
    OptionMarketMustBeMintAuthority,
    #[msg("OptionMarket must own the underlying asset pool")]
    OptionMarketMustOwnUnderlyingAssetPool,
    #[msg("OptionMarket must own the quote asset pool")]
    OptionMarketMustOwnQuoteAssetPool,
    #[msg("Stop trying to spoof the SPL Token program! Shame on you")]
    ExpectedSPLTokenProgramId,
    #[msg("Mint fee account must be owned by the FEE_OWNER")]
    MintFeeMustBeOwnedByFeeOwner,
    #[msg("Exercise fee account must be owned by the FEE_OWNER")]
    ExerciseFeeMustBeOwnedByFeeOwner,
    #[msg("Mint fee token must be the same as the underlying asset")]
    MintFeeTokenMustMatchUnderlyingAsset,
    #[msg("Exercise fee token must be the same as the quote asset")]
    ExerciseFeeTokenMustMatchQuoteAsset,
    #[msg("OptionMarket is expired, can't mint")]
    OptionMarketExpiredCantMint,
    #[msg("Underlying pool account does not match the value on the OptionMarket")]
    UnderlyingPoolAccountDoesNotMatchMarket,
    #[msg("OptionToken mint does not match the value on the OptionMarket")]
    OptionTokenMintDoesNotMatchMarket,
    #[msg("WriterToken mint does not match the value on the OptionMarket")]
    WriterTokenMintDoesNotMatchMarket,
    #[msg("MintFee key does not match the value on the OptionMarket")]
    MintFeeKeyDoesNotMatchOptionMarket,
    #[msg("The size argument must be > 0")]
    SizeCantBeLessThanEqZero,
    #[msg("exerciseFee key does not match the value on the OptionMarket")]
    ExerciseFeeKeyDoesNotMatchOptionMarket,
    #[msg("Quote pool account does not match the value on the OptionMarket")]
    QuotePoolAccountDoesNotMatchMarket,
    #[msg("Underlying destination mint must match underlying asset mint address")]
    UnderlyingDestMintDoesNotMatchUnderlyingAsset,
    #[msg("Fee owner does not match the program's fee owner")]
    FeeOwnerDoesNotMatchProgram,
    #[msg("OptionMarket is expired, can't exercise")]
    OptionMarketExpiredCantExercise,
    #[msg("OptionMarket has not expired, can't close")]
    OptionMarketNotExpiredCantClose,
    #[msg("Not enough assets in the quote asset pool")]
    NotEnoughQuoteAssetsInPool,
    #[msg("Invalid auth token provided")]
    InvalidAuth,
    #[msg("Coin mint must match option mint")]
    CoinMintIsNotOptionMint,
    #[msg("Cannot prune the market while it's still active")]
    CannotPruneActiveMarket,
    #[msg("Numerical overflow")]
    NumberOverflow,
    #[msg("Invalid order type")]
    InvalidOrderType,
    #[msg("Invalid self trade behavior")]
    InvalidSelfTradeBehavior,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Insufficient collateral to write options.")]
    InsufficientCollateralForWriting,
    #[msg("Insufficient Vault tokens to redeem.")]
    InsufficientVaultTokens,
    #[msg("Options contract is expired.")]
    ContractExpired,
    #[msg("Cannot redeem until contract expiry.")]
    ContractNotYetExpired,
    #[msg("mint amount was 0, skipping mint_helper()...")]
    InvalidMintAmount,
    #[msg("invalid time to exit position rebalanceExit()")]
    InvalidRebalanceExitTime,
    #[msg("invalid time to enter position rebalanceEnter()")]
    InvalidRebalanceEntryTime,
    #[msg("invalid time to call rebalancePrepare()")]
    InvalidRebalancePrepareTime,
    #[msg("invalid time to withdraw")]
    InvalidWithdrawalTime,
    #[msg("invalid time to deposit")]
    InvalidDepositTime,
    #[msg("invalid time to set next option")]
    InvalidSetNextOptionTime,
    #[msg("invalid deposit amount")]
    InvalidDepositAmount,
    #[msg("invalid rebalance settle time")]
    InvalidRebalanceSettleTime,
    #[msg("invalid rebalance settle state")]
    InvalidRebalanceSettleState,
    #[msg("invalid rebalance enter state")]
    InvalidRebalanceEnterState,
    #[msg("options position not settled, must be before withdrawal")]
    OptionsPositionNotSettled,
    #[msg("non underlying pools have assets when attempting withdraw")]
    NonUnderlyingPoolsHaveAssets,

    #[msg("volt must be vault mint authority")]
    VaultAuthorityMustBeVaultMintAuthority,
    #[msg("volt must own deposit pool")]
    VaultAuthorityMustOwnDepositPool,
    #[msg("volt must own premium pool")]
    VaultAuthorityMustOwnPremiumPool,
    #[msg("volt must own writer token pool")]
    VoltVaulttMustOwnWriterTokenPool,
    #[msg("volt must own option pool")]
    VoltVaultMustOwnOptionPool,

    #[msg("DepositPoolDoesNotMatchVoltVault")]
    DepositPoolDoesNotMatchVoltVault,
    #[msg("OptionPoolDoesNotMatchVoltVault")]
    OptionPoolDoesNotMatchVoltVault,
    #[msg("PremiumPoolDoesNotMatchVoltVault")]
    PremiumPoolDoesNotMatchVoltVault,
    #[msg("TradingPoolDoesNotMatchVoltVault")]
    TraidngPoolDoesNotMatchVoltVault,
    #[msg("option mint does not match option market")]
    OptionMintDoesNotMatchOptionMarket,

    #[msg("NoBidsInOptionOrderBook")]
    NoOrdersInOptionOrderBook,

    #[msg("cpi program must be Some in place order")]
    CpiProgramMustBeSomeInPlaceOrder,

    #[msg("new option must not be expired")]
    NewOptionMustNotBeExpired,
    #[msg("new option has roughly target expiry (within lower/upper bounds)")]
    NewOptionMustHaveExactExpiry,
    #[msg("new option has wrong underlying asset")]
    NewOptionHasWrongUnderlyingAsset,
    #[msg("new option has wrong quote asset")]
    NewOptionHasWrongQuoteAsset,
    #[msg("new option has wrong contract size")]
    NewOptionHasWrongContractSize,
    #[msg("new option has invalid strike")]
    NewOptionHasInvalidStrike,
    #[msg("rebalance settle has leftover writer tokens")]
    RebalanceSettleHasLeftoverWriterTokens,
    #[msg("current option must not be expired")]
    CurrentOptionMustNotBeExpired,
    #[msg("cannot reinitialize an (already initialized) volt")]
    CannotReinitializeVolt,
    #[msg("cannot reinitialize an (already initialized) volt")]
    OldOptionAndWriterTokenPoolsMustBeEmpty,
    #[msg("invalid old option writer token pools")]
    InvalidOldOptionWriterTokenPools,

    #[msg("vault mint does not match user token account")]
    VaultMintDoesNotMatchUserTokenAccount,
    #[msg("deposit pool mint does not match user token account")]
    DepositPoolMintDoesNotMatchUserTokenAccount,
    #[msg("vault authority does not match")]
    VaultAuthorityDoesNotMatch,
    #[msg("DEX program id does not match")]
    DexProgramIdDoesNotMatchAnchor,
    #[msg("Inertia program id does not match")]
    InertiaProgramIdDoesNotMatch,
    #[msg("Invalid authority for permissioned instruction")]
    InvalidAuthorityForPermissionedInstruction,
    #[msg("writer token mint does not match option market")]
    WriterTokenMintDoesNotMatchOptionMarket,
    #[msg("option market should be owned by protocol (e.g inertia)")]
    OptionMarketMustBeOwnedByProtocol,
    #[msg("underlying asset mint does not match voltvault")]
    UnderlyingAssetMintDoesNotMatchVoltVault,
    #[msg("quote asset mint does not match voltvault")]
    QuoteAssetMintDoesNotMatchVoltVault,
    #[msg("vault mint does not match volt vault")]
    VaultMintDoesNotMatchVoltVault,
    #[msg("option market does not match volt vault")]
    OptionMarketDoesNotMatchVoltVault,
    #[msg("writer token pool does not match volt vault")]
    WriterTokenPoolDoesNotMatchVoltVault,
    #[msg("invalid rebalance swap premium state")]
    InvalidRebalanceSwapPremiumState,
    #[msg("should be unreachable code")]
    ShouldBeUnreachable,
    #[msg("shouldn't have multiple pending deposits")]
    CantHaveMultiplePendingDeposits,
    #[msg("invalid start round state")]
    InvalidStartRoundState,
    #[msg("invalid set next option state")]
    InvalidSetNextOptionState,
    #[msg("invalid claim pending state")]
    InvalidClaimPendingState,
    #[msg("invalid end round state")]
    InvalidEndRoundState,
    #[msg("shouldn't have multiple pending deposits")]
    CantHaveMultiplePendingWithdrawals,
    #[msg("invalid claim pending withdrawal state")]
    InvalidClaimPendingWithdrawalState,
    #[msg("invalid next option market")]
    InvalidNextOptionMarket,
    #[msg("Auth token not revoked")]
    TokenNotRevoked,
    #[msg("user is not whitelisted")]
    NonWhitelistedUser,
    #[msg("user is not signer")]
    UserIsNotSigner,
    #[msg("authority does not match whitelist admin")]
    InvalidWhitelistAuthority,
    #[msg("whitelist and option market do not generate correct PDA")]
    InvalidWhitelistAndOptionMarketCombination,
    #[msg("round volt tokens mint does not match volt vault")]
    RoundVoltTokensMintDoesNotMatchVoltVault,
    #[msg("round underlying tokens mint does not match volt vault")]
    RoundUnderlyingTokensMintDoesNotMatchVoltVault,
    #[msg("UnderlyingAssetPoolDoesNotMatchOptionMarket")]
    UnderlyingAssetPoolDoesNotMatchOptionMarket,
    #[msg("no opposite order on serum market")]
    NoOppositeOrderOnSerumMarket,
    #[msg("bid price on serum market too low")]
    BidPriceOnSerumMarketTooLow,
    #[msg("offer price on serum market too high")]
    OfferPriceOnSerumMarketTooHigh,
    #[msg("underlying open orders does not match volt vault")]
    UnderlyingOpenOrdersDoesNotMatchVoltVault,
    #[msg("must have at least one market maker access token")]
    MustHaveAtLeastOneMarketMakerAccessToken,
    #[msg("middleware program id does not match expected")]
    MiddlewareProgramIdDoesNotMatch,
    #[msg("fee account owner does not match expected")]
    FeeAccountOwnerDoesNotMatch,
    #[msg("fee account mint does not match deposit pool")]
    FeeAccountMintDoesNotMatchDepositPool,
    #[msg("vault capacity would be exceeded")]
    VaultCapacityWouldBeExceeded,
    #[msg("individual deposit capacity would be exceeded")]
    IndividualDepositCapacityWouldBeExceeded,
    #[msg("unsupported option market program ID")]
    UnsupportedOptionMarketProgramId,
    #[msg("invalid end dca round state")]
    InvalidEndDcaRoundState,
    #[msg("round has not started")]
    RoundHasNotStarted,
    #[msg("permissioned makret premium pool does not match volt")]
    PermissionedMarketPremiumPoolDoesNotMatchVoltVault,
    #[msg("token account owners do not match")]
    TokenAccountOwnersDoNotMatch,
    #[msg("invalid permissioned market premium mint")]
    InvalidPermissionedMarketPremiumMint,
    #[msg("premium pool amount must be greater than zero")]
    PremiumPoolAmountMustBeGreaterThanZero,
    #[msg("can't close non empty token account")]
    CantCloseNonEmptyTokenAccount,
    #[msg("must finish entering before settling permissioned market premium funds")]
    MustFinishEnteringBeforeSettlingPermissionedMarketPremium,
    #[msg("pending withdrawal info must be initialized")]
    PendingWithdrawalInfoNotInitialized,
    #[msg("pending withdrawal does not exist")]
    PendingWithdrawalDoesNotExist,
    #[msg("cannot cancel pending withdrawal from old round")]
    CannotCancelPendingWithdrawalFromOldRound,
    #[msg("invalid take pending withdrawal fees state")]
    InvalidTakePendingWithdrawalFeesState,
    #[msg("pending deposit info not initialized")]
    PendingDepositInfoNotInitialized,
    #[msg("pending deposits does not exist")]
    PendingDepositDoesNotExist,
    #[msg("cannot cancel pending deposit from old round")]
    CannotCancelPendingDepositFromOldRound,
    #[msg("vault destination does not match volt vault")]
    VaultDestinationDoesNotMatchVoltVault,
    #[msg("must take withdrawal fees before starting round")]
    MustTakeWithdrawalFeesBeforeStartingRound,
    #[msg("round must be ended")]
    RoundMustBeEnded,
    #[msg("must not have sold option tokens to reset")]
    MustNotHaveSoldOptionTokens,
    #[msg("cannot close account unless empty")]
    CantCloseAccountUnlessEmpty,
    #[msg("open orders must be empty to close")]
    OpenOrderMustBeEmptyToClose,
    #[msg("invalid whitelist account (vector)")]
    InvalidWhitelistAccountVector,
    #[msg("invalid dao program ID")]
    InvalidDaoProgramId,
    #[msg("volt must be for dao")]
    VoltMustBeForDao,
    #[msg("invalid dao authority")]
    InvalidDaoAuthority,
    #[msg("dao authority must sign")]
    DaoAuthorityMustSign,
    #[msg("invalid pending deposit key")]
    InvalidPendingDepositKey,
    #[msg("invalid authority check")]
    InvalidAuthorityCheck,
    #[msg("entropy: invalid end entropy round state")]
    InvalidEndEntropyRoundState,
    #[msg("invalid volt type")]
    InvalidVoltType,
    #[msg("can't find perp market index")]
    CantFindPerpMarketIndex,
    #[msg("account equity less than zero")]
    AccountEquityLessThanZero,
    #[msg("quote position changed too much")]
    QuotePositionChangedTooMuch,
    #[msg("must move closer to target collateralization")]
    MustMoveCloserToTargetCollateralization,
    #[msg("collateral not within lenience")]
    CollateralNotWithinLenience,
    #[msg("invalid rebalance entropy state")]
    InvalidRebalanceEntropyState,
    #[msg("volt must have negative base position (be short)")]
    BasePositionMustBeNegative,
    #[msg("volt must have positive quote position (be short)")]
    QuotePositionMustBePositive,
    #[msg("target collateral ratio must be neggat")]
    TargetCollateralRatioMustBeNegative,
    #[msg("new equity must be higher than deposit amt")]
    NewEquityMustBeHigherThanDepositAmount,
    #[msg("instant transfers must be enabled")]
    InstantTransfersMustBeDisabled,
    #[msg("rebalance must be ready")]
    RebalanceMustBeReady,
    #[msg("spot hedge unbalanced")]
    IncorrectHedge,
    #[msg("vault name must be zero length")]
    VaultNameMustBeNonZeroLength,
    #[msg("vault does not support over leveraged strategies")]
    VaultDoesNotSupportOverLeveragedStrategies,
    #[msg("lenience must be greater than zero")]
    LenienceMustBeGreaterThanZero,
    #[msg("lenience should not be greater than leverage")]
    LenienceShouldNotBeGreaterThanLeverage,
    #[msg("hedge lenience should be greater than leverage")]
    HedgeLenienceMustBeGreaterThanZero,
    #[msg("exit early ratio must be < 1.0")]
    VaultDoesNotSupportExitEarlyOverLeveragedStrategies,
    #[msg("round number must not overflow")]
    RoundNumberMustNotOverflow,
    #[msg("invalid whitelist token account mint")]
    InvalidWhitelistTokenAccountMint,
    #[msg("soloptions program id does not matchf")]
    SoloptionsProgramIdDoesNotMatch,
    #[msg("whitelist token account owner is not user")]
    WhitelistTokenAccountOwnerIsNotUser,
    #[msg("sol transfer authority must be owned by volt program")]
    SolTransferAuthorityMustNotBeOwnedByVoltProgram,
    #[msg("Insufficient collateral to deposit.")]
    InsufficientCollateralForDeposit,
    #[msg("sol transfer authority must be writable/signer")]
    SolTransferAuthorityMustBeWritableAndSigner,
    #[msg("volt must be entropy type")]
    VoltMustBeOfEntropyType,
    #[msg("volt must be of short options type")]
    VoltMustBeofShortOptionsType,
    #[msg("deposits and withdrawals are turned off")]
    DepositsAndWithdrawalsAreTurnedOff,
    #[msg("unrecognized entropy program id")]
    UnrecognizedEntropyProgramId,
    #[msg("invalid take performance fees state")]
    InvalidTakePerformanceFeesState,
    #[msg("discriminator does not match")]
    DiscriminatorDoesNotMatch,
    #[msg("realized oracle price too far off client provided")]
    RealizedOraclePriceTooFarOffClientProvided,
    #[msg("vault mint supply must be zero if equity is zero")]
    VaultMintSupplyMustBeZeroIfEquityIsZero,
    #[msg("invalid setup rebalance entropy state")]
    InvalidSetupRebalanceEntropyState,
}
