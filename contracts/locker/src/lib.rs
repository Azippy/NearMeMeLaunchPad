use near_sdk::{
    env,
    near,
    AccountId,
    PanicOnDefault,
};

use std::collections::HashMap;

#[derive(
    near_sdk::borsh::BorshSerialize,
    near_sdk::borsh::BorshDeserialize,
    near_sdk::serde::Serialize,
    Clone,
)]
#[cfg_attr(
    not(target_arch = "wasm32"),
    derive(near_sdk::borsh::BorshSchema, schemars::JsonSchema)
)]
#[borsh(crate = "near_sdk::borsh")]
#[serde(crate = "near_sdk::serde")]
pub struct LiquidityLock {
    pub lock_id: String,

    pub launch_id: String,

    pub creator: AccountId,

    pub token_contract_id: AccountId,

    pub quote_asset_id: String,

    pub dex_id: String,

    pub position_id: String,

    pub locked_at: u64,

    pub unlock_at: u64,

    pub active: bool,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Locker {
    pub owner_id: AccountId,

    pub factory_id: AccountId,

    pub locks: HashMap<String, LiquidityLock>,
}

#[near]
impl Locker {
    // =========================================================
    // INITIALIZATION
    // =========================================================

    #[init]
    pub fn new(
        owner_id: AccountId,
        factory_id: AccountId,
    ) -> Self {
        assert!(
            !env::state_exists(),
            "Locker is already initialized"
        );

        Self {
            owner_id,
            factory_id,
            locks: HashMap::new(),
        }
    }

    // =========================================================
    // CREATE LOCK
    // =========================================================

    pub fn create_lock(
        &mut self,

        lock_id: String,

        launch_id: String,

        creator: AccountId,

        token_contract_id: AccountId,

        quote_asset_id: String,

        dex_id: String,

        position_id: String,

        unlock_at: u64,
    ) {
        self.assert_factory();

        assert!(
            !lock_id.is_empty(),
            "Lock ID is required"
        );

        assert!(
            !launch_id.is_empty(),
            "Launch ID is required"
        );

        assert!(
            !position_id.is_empty(),
            "Liquidity position ID is required"
        );

        assert!(
            unlock_at > env::block_timestamp(),
            "Unlock time must be in the future"
        );

        assert!(
            !self.locks.contains_key(&lock_id),
            "Lock already exists"
        );

        let lock = LiquidityLock {
            lock_id: lock_id.clone(),

            launch_id,

            creator,

            token_contract_id,

            quote_asset_id,

            dex_id,

            position_id,

            locked_at: env::block_timestamp(),

            unlock_at,

            active: true,
        };

        self.locks.insert(lock_id, lock);
    }

    // =========================================================
    // RELEASE LOCK
    // =========================================================

    pub fn release_lock(
        &mut self,
        lock_id: String,
    ) {
        self.assert_factory();

        let lock = self
            .locks
            .get_mut(&lock_id)
            .expect("Liquidity lock not found");

        assert!(
            lock.active,
            "Liquidity lock is already inactive"
        );

        assert!(
            env::block_timestamp() >= lock.unlock_at,
            "Liquidity is still locked"
        );

        lock.active = false;
    }

    // =========================================================
    // VIEW LOCK
    // =========================================================

    pub fn get_lock(
        &self,
        lock_id: String,
    ) -> Option<LiquidityLock> {
        self.locks.get(&lock_id).cloned()
    }

    // =========================================================
    // CHECK ACTIVE LOCK
    // =========================================================

    pub fn is_locked(
        &self,
        lock_id: String,
    ) -> bool {
        match self.locks.get(&lock_id) {
            Some(lock) => {
                lock.active
                    && env::block_timestamp() < lock.unlock_at
            }

            None => false,
        }
    }

    // =========================================================
    // CHECK IF UNLOCKED
    // =========================================================

    pub fn is_unlocked(
        &self,
        lock_id: String,
    ) -> bool {
        match self.locks.get(&lock_id) {
            Some(lock) => {
                env::block_timestamp() >= lock.unlock_at
            }

            None => false,
        }
    }

    // =========================================================
    // OWNER: CHANGE FACTORY
    // =========================================================

    pub fn set_factory(
        &mut self,
        factory_id: AccountId,
    ) {
        self.assert_owner();

        self.factory_id = factory_id;
    }

    // =========================================================
    // OWNER: CHANGE OWNER
    // =========================================================

    pub fn set_owner(
        &mut self,
        owner_id: AccountId,
    ) {
        self.assert_owner();

        self.owner_id = owner_id;
    }

    // =========================================================
    // INTERNAL SECURITY
    // =========================================================

    fn assert_factory(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.factory_id,
            "Only Factory can perform this action"
        );
    }

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only Locker owner can perform this action"
        );
    }
}