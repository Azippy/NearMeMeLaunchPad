use near_sdk::AccountId;

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    Clone,
    Debug,
    PartialEq,
)]
#[borsh(crate = "near_sdk::borsh")]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    Clone,
    Debug,
    PartialEq,
)]
#[borsh(crate = "near_sdk::borsh")]
pub enum TradeStatus {
    Pending,
    Settling,
    FeePending,
    Successful,
    Failed,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    Clone,
)]
#[borsh(crate = "near_sdk::borsh")]
pub struct PendingTrade {
    pub trade_id: String,
    pub launch_id: String,
    pub trader: AccountId,
    pub side: TradeSide,

    pub gross_input: u128,
    pub effective_input: u128,
    pub output_amount: u128,
    pub reserve_output: u128,

    pub trading_fee: u128,
    pub creator_tax: u128,

    pub platform_trading_fee: u128,
    pub creator_trading_fee: u128,

    pub platform_tax: u128,
    pub creator_tax_reward: u128,
    pub expected_results: u64,
    pub platform_trading_fee_paid: bool,
    pub platform_tax_paid: bool,
    pub creator_trading_fee_paid: bool,
    pub creator_tax_reward_paid: bool,

    pub status: TradeStatus,
}