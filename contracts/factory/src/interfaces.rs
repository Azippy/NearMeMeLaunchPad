#![allow(dead_code)]

use near_sdk::{
    ext_contract,
    json_types::U128,
    AccountId,
    Promise,
};

#[ext_contract(ext_treasury)]
pub trait TreasuryInterface {
    fn receive_launch_fee(&mut self) -> Promise;

    fn receive_trading_fee(&mut self) -> Promise;

    fn receive_trading_tax(&mut self) -> Promise;
}

#[ext_contract(ext_rewards)]
pub trait RewardsInterface {
    fn add_trading_fee_reward(
        &mut self,
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) -> Promise;

    fn add_trading_tax_reward(
        &mut self,
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) -> Promise;
}

#[ext_contract(ext_token)]
pub trait TokenInterface {
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    ) -> Promise;
}

#[ext_contract(ext_factory)]
pub trait FactoryCallbacks {
    fn on_launch_fee_received(&mut self, launch_id: String) -> Promise;

    fn on_token_deployed(&mut self, launch_id: String, token_contract_id: AccountId) -> Promise;

    fn after_buy_token_transfer(&mut self, trade_id: String) -> Promise;

    fn after_sell_payout(&mut self, trade_id: String) -> Promise;

    fn finalize_trade_fees(&mut self, trade_id: String) -> Promise;
}