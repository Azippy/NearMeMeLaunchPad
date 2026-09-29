use near_sdk::{
    env,
    near,
    AccountId,
    NearToken,
    PanicOnDefault,
    Promise,
};
use near_sdk::collections::LookupMap;

const NATIVE_NEAR_ASSET: &str = "native.near";

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    Clone,
)]
#[borsh(crate = "near_sdk::borsh")]
pub struct RewardKey {
    pub creator: AccountId,
    pub token_contract_id: AccountId,
    pub quote_asset_id: String,
}

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    Clone,
    near_sdk::serde::Serialize,
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct CreatorReward {
    pub creator: AccountId,
    pub token_contract_id: AccountId,
    pub quote_asset_id: String,

    pub trading_fee_rewards: NearToken,
    pub trading_tax_rewards: NearToken,

    pub total_claimed: NearToken,
}

impl CreatorReward {
    fn new(
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) -> Self {
        Self {
            creator,
            token_contract_id,
            quote_asset_id,
            trading_fee_rewards: NearToken::from_yoctonear(0),
            trading_tax_rewards: NearToken::from_yoctonear(0),
            total_claimed: NearToken::from_yoctonear(0),
        }
    }
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Rewards {
    pub owner_id: AccountId,
    pub factory_id: AccountId,

    pub rewards: LookupMap<String, CreatorReward>,
}

#[near]
impl Rewards {
    #[init]
    pub fn new(
        owner_id: AccountId,
        factory_id: AccountId,
    ) -> Self {
        assert!(
            !env::state_exists(),
            "Rewards contract is already initialized"
        );

        Self {
            owner_id,
            factory_id,
            rewards: LookupMap::new(b"r"),
        }
    }

    // =========================================================
    // ADD TRADING FEE REWARD
    // =========================================================

    #[payable]
    pub fn add_trading_fee_reward(
        &mut self,
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) {
        self.assert_factory();

        assert!(
            quote_asset_id == NATIVE_NEAR_ASSET,
            "This method currently supports native NEAR rewards only"
        );

        let amount = env::attached_deposit();

        assert!(
            amount > NearToken::from_yoctonear(0),
            "Reward must be greater than zero"
        );

        let key = self.make_key(
            &creator,
            &token_contract_id,
            &quote_asset_id,
        );

        let mut reward = self
            .rewards
            .get(&key)
            .unwrap_or_else(|| {
                CreatorReward::new(
                    creator.clone(),
                    token_contract_id.clone(),
                    quote_asset_id.clone(),
                )
            });

        reward.trading_fee_rewards =
            reward.trading_fee_rewards.saturating_add(amount);

        self.rewards.insert(&key, &reward);
    }

    // =========================================================
    // ADD TRADING TAX REWARD
    // =========================================================

    #[payable]
    pub fn add_trading_tax_reward(
        &mut self,
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) {
        self.assert_factory();

        assert!(
            quote_asset_id == NATIVE_NEAR_ASSET,
            "This method currently supports native NEAR rewards only"
        );

        let amount = env::attached_deposit();

        assert!(
            amount > NearToken::from_yoctonear(0),
            "Reward must be greater than zero"
        );

        let key = self.make_key(
            &creator,
            &token_contract_id,
            &quote_asset_id,
        );

        let mut reward = self
            .rewards
            .get(&key)
            .unwrap_or_else(|| {
                CreatorReward::new(
                    creator.clone(),
                    token_contract_id.clone(),
                    quote_asset_id.clone(),
                )
            });

        reward.trading_tax_rewards =
            reward.trading_tax_rewards.saturating_add(amount);

        self.rewards.insert(&key, &reward);
    }

    // =========================================================
    // CLAIM REWARDS
    // =========================================================

    pub fn claim_rewards(
        &mut self,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) -> Promise {
        let creator = env::predecessor_account_id();

        assert!(
            quote_asset_id == NATIVE_NEAR_ASSET,
            "This method currently supports native NEAR rewards only"
        );

        let key = self.make_key(
            &creator,
            &token_contract_id,
            &quote_asset_id,
        );

        let mut reward = self
            .rewards
            .get(&key)
            .expect("No reward account found");

        let trading_fee = reward.trading_fee_rewards;
        let trading_tax = reward.trading_tax_rewards;

        let total = trading_fee.saturating_add(trading_tax);

        assert!(
            total > NearToken::from_yoctonear(0),
            "No rewards available to claim"
        );

        // Effects first.
        reward.trading_fee_rewards =
            NearToken::from_yoctonear(0);

        reward.trading_tax_rewards =
            NearToken::from_yoctonear(0);

        reward.total_claimed =
            reward.total_claimed.saturating_add(total);

        self.rewards.insert(&key, &reward);

        // Interaction second.
        Promise::new(creator).transfer(total)
    }

    // =========================================================
    // VIEW REWARD
    // =========================================================

    pub fn get_rewards(
        &self,
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) -> CreatorReward {
        let key = self.make_key(
            &creator,
            &token_contract_id,
            &quote_asset_id,
        );

        self.rewards
            .get(&key)
            .unwrap_or_else(|| {
                CreatorReward::new(
                    creator,
                    token_contract_id,
                    quote_asset_id,
                )
            })
    }

    // =========================================================
    // VIEW PENDING REWARDS
    // =========================================================

    pub fn get_pending_rewards(
        &self,
        creator: AccountId,
        token_contract_id: AccountId,
        quote_asset_id: String,
    ) -> NearToken {
        let reward = self.get_rewards(
            creator,
            token_contract_id,
            quote_asset_id,
        );

        reward
            .trading_fee_rewards
            .saturating_add(reward.trading_tax_rewards)
    }

    // =========================================================
    // UPDATE FACTORY
    // =========================================================

    pub fn set_factory(
        &mut self,
        factory_id: AccountId,
    ) {
        self.assert_owner();

        self.factory_id = factory_id;
    }

    // =========================================================
    // INTERNAL HELPERS
    // =========================================================

    fn make_key(
        &self,
        creator: &AccountId,
        token_contract_id: &AccountId,
        quote_asset_id: &String,
    ) -> String {
        format!(
            "{}:{}:{}",
            creator,
            token_contract_id,
            quote_asset_id
        )
    }

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only rewards owner can perform this action"
        );
    }

    fn assert_factory(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.factory_id,
            "Only Factory can add rewards"
        );
    }
}