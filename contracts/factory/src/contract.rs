mod curve;
mod interfaces;
mod trade;

use curve::{
    calculate_buy_with_virtual_reserves,
    calculate_sell_with_virtual_reserves,
    split_creator_tax,
    split_trading_fee,
};
use interfaces::{ext_factory, ext_rewards, ext_token, ext_treasury};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    env,
    json_types::U128,
    near,
    AccountId,
    Gas,
    NearToken,
    PanicOnDefault,
    Promise,
    PromiseOrValue,
};
use near_sdk::collections::LookupMap;
use trade::{PendingTrade, TradeSide, TradeStatus};

const MAX_TAX_BPS: u16 = 1_000;
const TOTAL_TRADING_FEE_BPS: u16 = 100;
const TOKEN_TOTAL_SUPPLY: u128 = 1_000_000_000u128 * 10u128.pow(24);
const NATIVE_NEAR_ASSET: &str = "native.near";
const MAX_PROMISE_RESULT_BYTES: usize = 1_024;

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(near_sdk::borsh::BorshSchema, schemars::JsonSchema)
)]
pub enum LaunchMode {
    DirectMarket,
    BondingCurve,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(near_sdk::borsh::BorshSchema, schemars::JsonSchema)
)]
pub enum LaunchStatus {
    FeePending,
    Pending,
    DeployingToken,
    PreparingLiquidity,
    Active,
    Graduating,
    Graduated,
    Failed,
    Cancelled,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    Clone,
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(near_sdk::borsh::BorshSchema, schemars::JsonSchema)
)]
pub struct PairAsset {
    pub asset_id: String,
    pub symbol: String,
    pub name: String,
    pub asset_type: String,
    pub decimals: u8,
    pub enabled: bool,
    pub creator_selectable: bool,
    pub verified: bool,
    pub is_native_near: bool,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    Clone,
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(near_sdk::borsh::BorshSchema, schemars::JsonSchema)
)]
pub struct BondingCurveState {
    pub quote_reserve: u128,
    pub token_reserve: u128,
    pub virtual_quote_reserve: u128,
    pub virtual_token_reserve: u128,
    pub graduation_market_cap: u128,
    pub graduation_liquidity: u128,
    pub trading_fee_bps: u16,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    near_sdk::serde::Serialize,
    near_sdk::serde::Deserialize,
    Clone,
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(near_sdk::borsh::BorshSchema, schemars::JsonSchema)
)]
pub struct Launch {
    pub launch_id: String,
    pub creator_id: AccountId,
    pub token_contract_id: Option<AccountId>,
    pub token_name: String,
    pub token_symbol: String,
    pub mode: LaunchMode,
    pub status: LaunchStatus,
    pub quote_asset_id: String,
    pub buy_tax_bps: u16,
    pub sell_tax_bps: u16,
    pub launch_fee: u128,
    pub bonding_curve: Option<BondingCurveState>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(near_sdk::serde::Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TradeQuote {
    pub launch_id: String,
    pub gross_input: U128,
    pub trading_fee: U128,
    pub creator_tax: U128,
    pub platform_trading_fee: U128,
    pub creator_trading_fee: U128,
    pub platform_tax: U128,
    pub creator_tax_reward: U128,
    pub output_amount: U128,
}

#[derive(near_sdk::serde::Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProtocolConfig {
    pub launch_fee_yocto: u128,
    pub trading_fee_bps: u16,
    pub trading_fee_platform_bps: u16,
    pub trading_tax_platform_bps: u16,
    pub max_buy_tax_bps: u16,
    pub max_sell_tax_bps: u16,
}

#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SellMessage {
    pub launch_id: String,
    pub min_quote_out: U128,
    pub deadline: u64,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Factory {
    pub owner_id: AccountId,
    pub treasury_contract_id: AccountId,
    pub rewards_contract_id: AccountId,
    pub locker_contract_id: AccountId,
    pub launch_fee: NearToken,
    pub trading_fee_platform_bps: u16,
    pub trading_fee_creator_bps: u16,
    pub trading_tax_platform_bps: u16,
    pub trading_tax_creator_bps: u16,
    pub max_buy_tax_bps: u16,
    pub max_sell_tax_bps: u16,
    pub pair_assets: LookupMap<String, PairAsset>,
    pub launches: LookupMap<String, Launch>,
    pub token_launches: LookupMap<AccountId, String>,
    pub pending_trades: LookupMap<String, PendingTrade>,
    pub active_trades: LookupMap<String, String>,
    pub token_wasm: Vec<u8>,
    pub trade_nonce: u64,
}

#[near]
impl Factory {
    #[init]
    pub fn new(
        owner_id: AccountId,
        treasury_contract_id: AccountId,
        rewards_contract_id: AccountId,
        locker_contract_id: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Factory is already initialized");
        Self {
            owner_id,
            treasury_contract_id,
            rewards_contract_id,
            locker_contract_id,
            launch_fee: NearToken::from_millinear(200),
            trading_fee_platform_bps: 3_000,
            trading_fee_creator_bps: 7_000,
            trading_tax_platform_bps: 4_000,
            trading_tax_creator_bps: 6_000,
            max_buy_tax_bps: MAX_TAX_BPS,
            max_sell_tax_bps: MAX_TAX_BPS,
            pair_assets: LookupMap::new(b"pair".to_vec()),
            launches: LookupMap::new(b"launch".to_vec()),
            token_launches: LookupMap::new(b"token_launch".to_vec()),
            pending_trades: LookupMap::new(b"trade".to_vec()),
            active_trades: LookupMap::new(b"active_trade".to_vec()),
            token_wasm: Vec::new(),
            trade_nonce: 0,
        }
    }

    pub fn get_protocol_config(&self) -> ProtocolConfig {
        ProtocolConfig {
            launch_fee_yocto: self.launch_fee.as_yoctonear(),
            trading_fee_bps: TOTAL_TRADING_FEE_BPS,
            trading_fee_platform_bps: self.trading_fee_platform_bps,
            trading_tax_platform_bps: self.trading_tax_platform_bps,
            max_buy_tax_bps: self.max_buy_tax_bps,
            max_sell_tax_bps: self.max_sell_tax_bps,
        }
    }

    #[payable]
    pub fn create_launch(
        &mut self,
        launch_id: String,
        token_name: String,
        token_symbol: String,
        mode: LaunchMode,
        quote_asset_id: String,
        buy_tax_bps: u16,
        sell_tax_bps: u16,
        initial_purchase_amount_yocto: u128,
    ) -> Promise {
        let creator_id = env::predecessor_account_id();
        assert!(!launch_id.trim().is_empty(), "Launch ID is required");
        assert!(!token_name.trim().is_empty(), "Token name is required");
        assert!(!token_symbol.trim().is_empty(), "Token symbol is required");
        assert!(buy_tax_bps <= self.max_buy_tax_bps, "Buy tax exceeds protocol maximum");
        assert!(sell_tax_bps <= self.max_sell_tax_bps, "Sell tax exceeds protocol maximum");
        assert!(self.launches.get(&launch_id).is_none(), "Launch ID already exists");
        assert_eq!(mode, LaunchMode::BondingCurve, "Direct market launches are not supported yet");
        assert!(initial_purchase_amount_yocto > 0, "Initial curve liquidity must be greater than zero");

        let pair = self.pair_assets.get(&quote_asset_id).expect("Quote asset is not approved");
        assert!(pair.enabled, "Quote asset is disabled");
        assert!(pair.creator_selectable, "Quote asset cannot be selected by creators");
        assert!(pair.is_native_near, "Only native NEAR launches are currently supported");
        assert_eq!(quote_asset_id, NATIVE_NEAR_ASSET, "Only native NEAR is supported");

        let initial_purchase = NearToken::from_yoctonear(initial_purchase_amount_yocto);
        let required = self.launch_fee.saturating_add(initial_purchase);
        assert_eq!(env::attached_deposit(), required, "Attached deposit must equal launch fee plus initial purchase");

        let now = env::block_timestamp();
        let bonding_curve = match mode {
            LaunchMode::BondingCurve => Some(BondingCurveState {
                quote_reserve: initial_purchase_amount_yocto,
                token_reserve: TOKEN_TOTAL_SUPPLY,
                virtual_quote_reserve: 0,
                virtual_token_reserve: 0,
                graduation_market_cap: 0,
                graduation_liquidity: 0,
                trading_fee_bps: TOTAL_TRADING_FEE_BPS,
            }),
            LaunchMode::DirectMarket => None,
        };
        let launch = Launch {
            launch_id: launch_id.clone(),
            creator_id,
            token_contract_id: None,
            token_name,
            token_symbol,
            mode,
            status: LaunchStatus::FeePending,
            quote_asset_id,
            buy_tax_bps,
            sell_tax_bps,
            launch_fee: self.launch_fee.as_yoctonear(),
            bonding_curve,
            created_at: now,
            updated_at: now,
        };
        self.launches.insert(&launch_id, &launch);

        ext_treasury::ext(self.treasury_contract_id.clone())
            .with_attached_deposit(self.launch_fee)
            .with_static_gas(Gas::from_tgas(10))
            .receive_launch_fee()
            .then(ext_factory::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(10))
                .on_launch_fee_received(launch_id))
    }

    pub fn get_launch(&self, launch_id: String) -> Option<Launch> {
        self.launches.get(&launch_id)
    }

    pub fn get_launch_by_token(&self, token_contract_id: AccountId) -> Option<String> {
        self.token_launches.get(&token_contract_id)
    }

    pub fn get_pair_asset(&self, asset_id: String) -> Option<PairAsset> {
        self.pair_assets.get(&asset_id)
    }

    pub fn add_pair_asset(
        &mut self,
        asset_id: String,
        symbol: String,
        name: String,
        asset_type: String,
        decimals: u8,
        creator_selectable: bool,
        verified: bool,
        is_native_near: bool,
    ) {
        self.assert_owner();
        assert!(!asset_id.trim().is_empty(), "Asset ID is required");
        assert!(self.pair_assets.get(&asset_id).is_none(), "Pair asset already exists");
        self.pair_assets.insert(&asset_id, &PairAsset {
            asset_id: asset_id.clone(),
            symbol,
            name,
            asset_type,
            decimals,
            enabled: true,
            creator_selectable,
            verified,
            is_native_near,
        });
    }

    pub fn set_pair_asset_enabled(&mut self, asset_id: String, enabled: bool) {
        self.assert_owner();
        let mut asset = self.pair_assets.get(&asset_id).expect("Pair asset not found");
        asset.enabled = enabled;
        self.pair_assets.insert(&asset_id, &asset);
    }

    pub fn set_launch_fee(&mut self, launch_fee_yocto: u128) {
        self.assert_owner();
        assert!(launch_fee_yocto > 0, "Launch fee must be greater than zero");
        self.launch_fee = NearToken::from_yoctonear(launch_fee_yocto);
    }

    pub fn set_treasury(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.treasury_contract_id = account_id;
    }

    pub fn set_rewards(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.rewards_contract_id = account_id;
    }

    pub fn set_locker(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.locker_contract_id = account_id;
    }

    pub fn set_owner(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.owner_id = account_id;
    }

    pub fn set_token_wasm(&mut self, wasm: Vec<u8>) {
        self.assert_owner();
        assert!(!wasm.is_empty(), "Token WASM cannot be empty");
        self.token_wasm = wasm;
    }

    #[payable]
    pub fn deploy_token_for_launch(&mut self, launch_id: String) -> Promise {
        let caller = env::predecessor_account_id();
        let mut launch = self.launches.get(&launch_id).expect("Launch not found");
        assert_eq!(launch.creator_id, caller, "Only the creator can deploy this token");
        assert_eq!(launch.status, LaunchStatus::Pending, "Launch is not pending");
        assert!(!self.token_wasm.is_empty(), "Token WASM has not been configured");
        assert_eq!(env::attached_deposit(), NearToken::from_near(1), "Exactly 1 NEAR is required for token account creation");

        let prefix: String = launch.launch_id.to_lowercase().chars()
            .filter(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || *character == '-' || *character == '_')
            .take(32)
            .collect();
        assert!(prefix.len() >= 2, "Launch ID must contain at least two account-name characters");
        let token_contract_id = format!("{}.{}", prefix, env::current_account_id())
            .parse::<AccountId>()
            .expect("Invalid token account ID");
        assert!(self.token_launches.get(&token_contract_id).is_none(), "Token account is already assigned");

        launch.status = LaunchStatus::DeployingToken;
        launch.updated_at = env::block_timestamp();
        self.launches.insert(&launch_id, &launch);
        self.token_launches.insert(&token_contract_id, &launch_id);

        let current = env::current_account_id();
        Promise::new(token_contract_id.clone())
            .create_account()
            .transfer(NearToken::from_near(1))
            .deploy_contract(self.token_wasm.clone())
            .function_call(
                "new".to_string(),
                near_sdk::serde_json::to_vec(&near_sdk::serde_json::json!({
                    "owner_id": current,
                    "creator_id": launch.creator_id,
                    "launch_vault_id": env::current_account_id(),
                    "factory_contract_id": env::current_account_id(),
                    "name": launch.token_name,
                    "symbol": launch.token_symbol,
                    "icon": null,
                    "buy_tax_bps": launch.buy_tax_bps,
                    "sell_tax_bps": launch.sell_tax_bps
                })).expect("Failed to serialize token initialization"),
                NearToken::from_yoctonear(0),
                Gas::from_tgas(50),
            )
            .then(ext_factory::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(10))
                .on_token_deployed(launch_id, token_contract_id))
    }

    #[private]
    pub fn on_token_deployed(&mut self, launch_id: String, token_contract_id: AccountId) -> bool {
        let mut launch = self.launches.get(&launch_id).expect("Launch not found");
        if !matches!(env::promise_result_checked(0, MAX_PROMISE_RESULT_BYTES), Ok(_)) {
            launch.status = LaunchStatus::Failed;
            launch.updated_at = env::block_timestamp();
            self.launches.insert(&launch_id, &launch);
            self.token_launches.remove(&token_contract_id);
            let initial_purchase = launch.bonding_curve
                .as_ref()
                .map(|curve| curve.quote_reserve)
                .unwrap_or(0);
            let refund = initial_purchase
                .checked_add(NearToken::from_near(1).as_yoctonear())
                .expect("Deployment refund overflow");
            Promise::new(launch.creator_id)
                .transfer(NearToken::from_yoctonear(refund))
                .detach();
            return false;
        }
        launch.token_contract_id = Some(token_contract_id.clone());
        launch.status = LaunchStatus::PreparingLiquidity;
        launch.updated_at = env::block_timestamp();
        self.launches.insert(&launch_id, &launch);
        self.token_launches.insert(&token_contract_id, &launch_id);
        true
    }

    #[private]
    pub fn on_launch_fee_received(&mut self, launch_id: String) -> bool {
        let mut launch = self.launches.get(&launch_id).expect("Launch not found");
        assert_eq!(launch.status, LaunchStatus::FeePending, "Launch fee is not pending");
        let fee_paid = env::promise_results_count() == 1
            && env::promise_result_checked(0, MAX_PROMISE_RESULT_BYTES).is_ok();
        if fee_paid {
            launch.status = LaunchStatus::Pending;
            launch.updated_at = env::block_timestamp();
            self.launches.insert(&launch_id, &launch);
            return true;
        }

        self.launches.remove(&launch_id);
        let initial_purchase = launch.bonding_curve
            .as_ref()
            .map(|curve| curve.quote_reserve)
            .unwrap_or(0);
        let refund = launch.launch_fee
            .checked_add(initial_purchase)
            .expect("Launch refund overflow");
        Promise::new(launch.creator_id)
            .transfer(NearToken::from_yoctonear(refund))
            .detach();
        false
    }

    pub fn mark_active(&mut self, launch_id: String) {
        self.assert_owner();
        let mut launch = self.launches.get(&launch_id).expect("Launch not found");
        assert!(launch.token_contract_id.is_some(), "Token is not deployed");
        assert_eq!(launch.status, LaunchStatus::PreparingLiquidity, "Launch is not ready to become active");
        launch.status = LaunchStatus::Active;
        launch.updated_at = env::block_timestamp();
        self.launches.insert(&launch_id, &launch);
    }

    pub fn quote_buy(&self, launch_id: String, quote_amount: U128) -> TradeQuote {
        let launch = self.launches.get(&launch_id).expect("Launch not found");
        assert_eq!(launch.status, LaunchStatus::Active, "Launch is not active");
        let curve = launch.bonding_curve.as_ref().expect("Launch is not a bonding curve");
        let result = calculate_buy_with_virtual_reserves(
            curve.quote_reserve,
            curve.token_reserve,
            curve.virtual_quote_reserve,
            curve.virtual_token_reserve,
            quote_amount.0,
            curve.trading_fee_bps,
            launch.buy_tax_bps,
        );
        self.make_quote(launch_id, quote_amount.0, result.amount_out, result.fee_amount, result.creator_tax_amount)
    }

    pub fn quote_sell(&self, launch_id: String, token_amount: U128) -> TradeQuote {
        let launch = self.launches.get(&launch_id).expect("Launch not found");
        assert_eq!(launch.status, LaunchStatus::Active, "Launch is not active");
        let curve = launch.bonding_curve.as_ref().expect("Launch is not a bonding curve");
        let result = calculate_sell_with_virtual_reserves(
            curve.token_reserve,
            curve.quote_reserve,
            curve.virtual_token_reserve,
            curve.virtual_quote_reserve,
            token_amount.0,
            curve.trading_fee_bps,
            launch.sell_tax_bps,
        );
        self.make_quote(launch_id, token_amount.0, result.amount_out, result.fee_amount, result.creator_tax_amount)
    }

    #[payable]
    pub fn buy(&mut self, launch_id: String, min_token_out: U128, deadline: u64) -> Promise {
        assert!(env::block_timestamp() <= deadline, "Trade deadline has expired");
        let buyer = env::predecessor_account_id();
        let gross_quote = env::attached_deposit().as_yoctonear();
        assert!(gross_quote > 0, "Buy amount must be greater than zero");
        let launch = self.launches.get(&launch_id).expect("Launch not found");
        assert_eq!(launch.status, LaunchStatus::Active, "Launch is not active");
        assert_eq!(launch.quote_asset_id, NATIVE_NEAR_ASSET, "Only native NEAR is supported");
        let token_contract_id = launch.token_contract_id.clone().expect("Token is not deployed");
        let curve = launch.bonding_curve.as_ref().expect("Launch is not a bonding curve");
        let result = calculate_buy_with_virtual_reserves(
            curve.quote_reserve,
            curve.token_reserve,
            curve.virtual_quote_reserve,
            curve.virtual_token_reserve,
            gross_quote,
            curve.trading_fee_bps,
            launch.buy_tax_bps,
        );
        assert!(result.amount_out >= min_token_out.0, "Slippage exceeded");
        assert!(result.amount_out <= curve.token_reserve, "Insufficient token reserves");

        let trade_id = self.new_trade_id();
        self.acquire_trade_lock(&launch_id, &trade_id);
        let (platform_trading_fee, creator_trading_fee) = split_trading_fee(result.fee_amount);
        let (platform_tax, creator_tax_reward) = split_creator_tax(result.creator_tax_amount);
        let effective_input = gross_quote.checked_sub(
            result.fee_amount.checked_add(result.creator_tax_amount).expect("Fee overflow")
        ).expect("Fees exceed input");
        let expected_results = u64::from(platform_trading_fee > 0)
            + u64::from(platform_tax > 0)
            + u64::from(creator_trading_fee > 0)
            + u64::from(creator_tax_reward > 0);
        let trade = PendingTrade {
            trade_id: trade_id.clone(),
            launch_id: launch_id.clone(),
            trader: buyer.clone(),
            side: TradeSide::Buy,
            gross_input: gross_quote,
            effective_input,
            output_amount: result.amount_out,
            reserve_output: result.amount_out,
            trading_fee: result.fee_amount,
            creator_tax: result.creator_tax_amount,
            platform_trading_fee,
            creator_trading_fee,
            platform_tax,
            creator_tax_reward,
            expected_results,
            platform_trading_fee_paid: platform_trading_fee == 0,
            platform_tax_paid: platform_tax == 0,
            creator_trading_fee_paid: creator_trading_fee == 0,
            creator_tax_reward_paid: creator_tax_reward == 0,
            status: TradeStatus::Settling,
        };
        self.pending_trades.insert(&trade_id, &trade);

        ext_token::ext(token_contract_id)
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(10))
            .ft_transfer(buyer, U128(result.amount_out), Some(format!("NearMeMePad buy {}", launch_id)))
            .then(ext_factory::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(15))
                .after_buy_token_transfer(trade_id))
    }

    #[private]
    pub fn after_buy_token_transfer(&mut self, trade_id: String) -> PromiseOrValue<bool> {
        let mut trade = self.pending_trades.get(&trade_id).expect("Trade not found");
        assert_eq!(self.active_trades.get(&trade.launch_id), Some(trade_id.clone()), "Trade lock does not match callback");
        let token_transfer_succeeded = env::promise_results_count() == 1
            && env::promise_result_checked(0, MAX_PROMISE_RESULT_BYTES).is_ok();
        if !token_transfer_succeeded {
            trade.status = TradeStatus::Failed;
            self.pending_trades.insert(&trade_id, &trade);
            self.active_trades.remove(&trade.launch_id);
            Promise::new(trade.trader)
                .transfer(NearToken::from_yoctonear(trade.gross_input))
                .detach();
            return PromiseOrValue::Value(false);
        }

        let mut launch = self.launches.get(&trade.launch_id).expect("Launch not found");
        let curve = launch.bonding_curve.as_mut().expect("Bonding curve not found");
        curve.quote_reserve = curve.quote_reserve.checked_add(trade.effective_input).expect("Quote reserve overflow");
        curve.token_reserve = curve.token_reserve.checked_sub(trade.output_amount).expect("Token reserve underflow");
        launch.updated_at = env::block_timestamp();
        self.launches.insert(&trade.launch_id, &launch);

        if trade.expected_results == 0 {
            trade.status = TradeStatus::Successful;
            self.pending_trades.insert(&trade_id, &trade);
            self.active_trades.remove(&trade.launch_id);
            return PromiseOrValue::Value(true);
        }
        trade.status = TradeStatus::Settling;
        self.pending_trades.insert(&trade_id, &trade);
        let fee_settlement = self.build_fee_settlement(&trade)
            .expect("Expected fee payments for this trade");
        PromiseOrValue::Promise(fee_settlement.then(
            ext_factory::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(15))
                .finalize_trade_fees(trade_id)
        ))
    }

    pub fn retry_trade_fees(&mut self, trade_id: String) -> Promise {
        self.assert_owner();
        let mut trade = self.pending_trades.get(&trade_id).expect("Trade not found");
        assert_eq!(trade.status, TradeStatus::FeePending, "Trade has no pending fee payments");
        trade.expected_results = Self::pending_fee_count(&trade);
        assert!(trade.expected_results > 0, "Trade has no unpaid fees");
        trade.status = TradeStatus::Settling;
        self.pending_trades.insert(&trade_id, &trade);
        self.build_fee_settlement(&trade)
            .expect("Expected fee payments for this trade")
            .then(ext_factory::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(15))
                .finalize_trade_fees(trade_id))
    }

    #[private]
    pub fn finalize_trade_fees(&mut self, trade_id: String) -> U128 {
        let mut trade = self.pending_trades.get(&trade_id).expect("Trade not found");
        let result_count_matches = env::promise_results_count() == trade.expected_results;
        let mut result_index = 0;
        if trade.platform_trading_fee > 0 && !trade.platform_trading_fee_paid {
            trade.platform_trading_fee_paid = result_count_matches
                && env::promise_result_checked(result_index, MAX_PROMISE_RESULT_BYTES).is_ok();
            result_index += 1;
        }
        if trade.platform_tax > 0 && !trade.platform_tax_paid {
            trade.platform_tax_paid = result_count_matches
                && env::promise_result_checked(result_index, MAX_PROMISE_RESULT_BYTES).is_ok();
            result_index += 1;
        }
        if trade.creator_trading_fee > 0 && !trade.creator_trading_fee_paid {
            trade.creator_trading_fee_paid = result_count_matches
                && env::promise_result_checked(result_index, MAX_PROMISE_RESULT_BYTES).is_ok();
            result_index += 1;
        }
        if trade.creator_tax_reward > 0 && !trade.creator_tax_reward_paid {
            trade.creator_tax_reward_paid = result_count_matches
                && env::promise_result_checked(result_index, MAX_PROMISE_RESULT_BYTES).is_ok();
        }
        let all_fees_paid = trade.platform_trading_fee_paid
            && trade.platform_tax_paid
            && trade.creator_trading_fee_paid
            && trade.creator_tax_reward_paid;
        trade.status = if all_fees_paid { TradeStatus::Successful } else { TradeStatus::FeePending };
        self.pending_trades.insert(&trade_id, &trade);
        self.active_trades.remove(&trade.launch_id);
        U128(0)
    }

    fn pending_fee_count(trade: &PendingTrade) -> u64 {
        u64::from(trade.platform_trading_fee > 0 && !trade.platform_trading_fee_paid)
            + u64::from(trade.platform_tax > 0 && !trade.platform_tax_paid)
            + u64::from(trade.creator_trading_fee > 0 && !trade.creator_trading_fee_paid)
            + u64::from(trade.creator_tax_reward > 0 && !trade.creator_tax_reward_paid)
    }

    fn build_fee_settlement(&self, trade: &PendingTrade) -> Option<Promise> {
        let mut settlement = None;
        if trade.platform_trading_fee > 0 && !trade.platform_trading_fee_paid {
            settlement = Some(ext_treasury::ext(self.treasury_contract_id.clone())
                .with_attached_deposit(NearToken::from_yoctonear(trade.platform_trading_fee))
                .with_static_gas(Gas::from_tgas(10)).receive_trading_fee());
        }
        if trade.platform_tax > 0 && !trade.platform_tax_paid {
            let promise = ext_treasury::ext(self.treasury_contract_id.clone())
                .with_attached_deposit(NearToken::from_yoctonear(trade.platform_tax))
                .with_static_gas(Gas::from_tgas(10)).receive_trading_tax();
            settlement = Some(match settlement { Some(previous) => previous.and(promise), None => promise });
        }
        if trade.creator_trading_fee > 0 && !trade.creator_trading_fee_paid {
            let token_id = self.launches.get(&trade.launch_id)
                .and_then(|launch| launch.token_contract_id)
                .expect("Token is not deployed");
            let creator = self.launches.get(&trade.launch_id)
                .expect("Launch not found").creator_id;
            let promise = ext_rewards::ext(self.rewards_contract_id.clone())
                .with_attached_deposit(NearToken::from_yoctonear(trade.creator_trading_fee))
                .with_static_gas(Gas::from_tgas(10))
                .add_trading_fee_reward(creator, token_id, NATIVE_NEAR_ASSET.to_string());
            settlement = Some(match settlement { Some(previous) => previous.and(promise), None => promise });
        }
        if trade.creator_tax_reward > 0 && !trade.creator_tax_reward_paid {
            let token_id = self.launches.get(&trade.launch_id)
                .and_then(|launch| launch.token_contract_id)
                .expect("Token is not deployed");
            let creator = self.launches.get(&trade.launch_id)
                .expect("Launch not found").creator_id;
            let promise = ext_rewards::ext(self.rewards_contract_id.clone())
                .with_attached_deposit(NearToken::from_yoctonear(trade.creator_tax_reward))
                .with_static_gas(Gas::from_tgas(10))
                .add_trading_tax_reward(creator, token_id, NATIVE_NEAR_ASSET.to_string());
            settlement = Some(match settlement { Some(previous) => previous.and(promise), None => promise });
        }
        settlement
    }

    fn make_quote(&self, launch_id: String, gross_input: u128, output: u128, fee: u128, tax: u128) -> TradeQuote {
        let (platform_trading_fee, creator_trading_fee) = split_trading_fee(fee);
        let (platform_tax, creator_tax_reward) = split_creator_tax(tax);
        TradeQuote {
            launch_id,
            gross_input: U128(gross_input),
            trading_fee: U128(fee),
            creator_tax: U128(tax),
            platform_trading_fee: U128(platform_trading_fee),
            creator_trading_fee: U128(creator_trading_fee),
            platform_tax: U128(platform_tax),
            creator_tax_reward: U128(creator_tax_reward),
            output_amount: U128(output),
        }
    }

    fn new_trade_id(&mut self) -> String {
        let nonce = self.trade_nonce;
        self.trade_nonce = self.trade_nonce.checked_add(1).expect("Trade nonce overflow");
        format!("{}-{nonce}", env::block_height())
    }

    fn acquire_trade_lock(&mut self, launch_id: &str, trade_id: &str) {
        assert!(self.active_trades.get(&launch_id.to_string()).is_none(), "A trade is already settling for this launch");
        self.active_trades.insert(&launch_id.to_string(), &trade_id.to_string());
    }

    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only Factory owner can perform this action");
    }
}

#[near]
impl FungibleTokenReceiver for Factory {
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128> {
        let message: SellMessage = near_sdk::serde_json::from_str(&msg).expect("Invalid sell message");
        assert!(env::block_timestamp() <= message.deadline, "Trade deadline has expired");
        let token_contract_id = env::predecessor_account_id();
        let launch = self.launches.get(&message.launch_id).expect("Launch not found");
        assert_eq!(launch.status, LaunchStatus::Active, "Launch is not active");
        assert_eq!(launch.token_contract_id.as_ref(), Some(&token_contract_id), "Wrong token contract");
        assert_eq!(launch.quote_asset_id, NATIVE_NEAR_ASSET, "Only native NEAR is supported");

        let curve = launch.bonding_curve.as_ref().expect("Launch is not a bonding curve");
        let result = calculate_sell_with_virtual_reserves(
            curve.token_reserve,
            curve.quote_reserve,
            curve.virtual_token_reserve,
            curve.virtual_quote_reserve,
            amount.0,
            curve.trading_fee_bps,
            launch.sell_tax_bps,
        );
        assert!(result.amount_out >= message.min_quote_out.0, "Slippage exceeded");
        assert!(result.amount_out > 0, "Sell output must be greater than zero");
        let raw_quote = result.amount_out.checked_add(result.fee_amount)
            .and_then(|value| value.checked_add(result.creator_tax_amount)).expect("Sell output overflow");
        assert!(raw_quote <= curve.quote_reserve, "Insufficient quote reserves");

        let trade_id = self.new_trade_id();
        self.acquire_trade_lock(&message.launch_id, &trade_id);
        let (platform_trading_fee, creator_trading_fee) = split_trading_fee(result.fee_amount);
        let (platform_tax, creator_tax_reward) = split_creator_tax(result.creator_tax_amount);
        let expected_results = u64::from(platform_trading_fee > 0)
            + u64::from(platform_tax > 0)
            + u64::from(creator_trading_fee > 0)
            + u64::from(creator_tax_reward > 0);
        let trade = PendingTrade {
            trade_id: trade_id.clone(),
            launch_id: message.launch_id.clone(),
            trader: sender_id.clone(),
            side: TradeSide::Sell,
            gross_input: amount.0,
            effective_input: amount.0,
            output_amount: result.amount_out,
            reserve_output: raw_quote,
            trading_fee: result.fee_amount,
            creator_tax: result.creator_tax_amount,
            platform_trading_fee,
            creator_trading_fee,
            platform_tax,
            creator_tax_reward,
            expected_results,
            platform_trading_fee_paid: platform_trading_fee == 0,
            platform_tax_paid: platform_tax == 0,
            creator_trading_fee_paid: creator_trading_fee == 0,
            creator_tax_reward_paid: creator_tax_reward == 0,
            status: TradeStatus::Settling,
        };
        self.pending_trades.insert(&trade_id, &trade);

        PromiseOrValue::Promise(
            Promise::new(sender_id)
                .transfer(NearToken::from_yoctonear(result.amount_out))
                .then(ext_factory::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(15))
                    .after_sell_payout(trade_id))
        )
    }
}

#[near]
impl Factory {
    #[private]
    pub fn after_sell_payout(&mut self, trade_id: String) -> PromiseOrValue<U128> {
        let mut trade = self.pending_trades.get(&trade_id).expect("Trade not found");
        assert_eq!(self.active_trades.get(&trade.launch_id), Some(trade_id.clone()), "Trade lock does not match callback");
        let payout_succeeded = env::promise_results_count() == 1
            && env::promise_result_checked(0, MAX_PROMISE_RESULT_BYTES).is_ok();
        if !payout_succeeded {
            trade.status = TradeStatus::Failed;
            self.pending_trades.insert(&trade_id, &trade);
            self.active_trades.remove(&trade.launch_id);
            return PromiseOrValue::Value(U128(trade.gross_input));
        }

        let mut launch = self.launches.get(&trade.launch_id).expect("Launch not found");
        let curve = launch.bonding_curve.as_mut().expect("Bonding curve not found");
        curve.token_reserve = curve.token_reserve.checked_add(trade.gross_input).expect("Token reserve overflow");
        curve.quote_reserve = curve.quote_reserve.checked_sub(trade.reserve_output).expect("Quote reserve underflow");
        launch.updated_at = env::block_timestamp();
        self.launches.insert(&trade.launch_id, &launch);

        if trade.expected_results == 0 {
            trade.status = TradeStatus::Successful;
            self.pending_trades.insert(&trade_id, &trade);
            self.active_trades.remove(&trade.launch_id);
            return PromiseOrValue::Value(U128(0));
        }
        trade.status = TradeStatus::Settling;
        self.pending_trades.insert(&trade_id, &trade);
        let fee_settlement = self.build_fee_settlement(&trade)
            .expect("Expected fee payments for this trade");
        PromiseOrValue::Promise(fee_settlement.then(
            ext_factory::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(15))
                .finalize_trade_fees(trade_id)
        ))
    }
}