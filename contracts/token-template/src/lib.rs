use near_contract_standards::fungible_token::{
    core::FungibleTokenCore,
    metadata::{
        FungibleTokenMetadata,
        FungibleTokenMetadataProvider,
        FT_METADATA_SPEC,
    },
    resolver::FungibleTokenResolver,
    FungibleToken,
};
use near_contract_standards::storage_management::StorageManagement;

use near_sdk::{
    env,
    near,
    AccountId,
    PanicOnDefault,
    PromiseOrValue,
};

const TOTAL_SUPPLY: u128 =
    1_000_000_000u128 * 10u128.pow(24);

const TOKEN_DECIMALS: u8 = 24;
const MAX_TAX_BPS: u16 = 1_000;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct NearMeMePadToken {
    pub owner_id: AccountId,
    pub creator_id: AccountId,
    pub launch_vault_id: AccountId,

    pub token: FungibleToken,

    pub metadata: FungibleTokenMetadata,

    pub buy_tax_bps: u16,
    pub sell_tax_bps: u16,

    pub factory_contract_id: AccountId,
}

#[near]
impl NearMeMePadToken {
    #[init]
    pub fn new(
        owner_id: AccountId,
        creator_id: AccountId,
        launch_vault_id: AccountId,
        factory_contract_id: AccountId,

        name: String,
        symbol: String,
        icon: Option<String>,

        buy_tax_bps: u16,
        sell_tax_bps: u16,
    ) -> Self {
        assert!(
            !env::state_exists(),
            "Token is already initialized"
        );

        assert!(!name.trim().is_empty(), "Token name is required");
        assert!(!symbol.trim().is_empty(), "Token symbol is required");

        assert!(
            buy_tax_bps <= MAX_TAX_BPS,
            "Buy tax cannot exceed 10%"
        );

        assert!(
            sell_tax_bps <= MAX_TAX_BPS,
            "Sell tax cannot exceed 10%"
        );

        let mut token =
            FungibleToken::new(b"t".to_vec());

        token.internal_register_account(
            &launch_vault_id
        );

        token.internal_deposit(
            &launch_vault_id,
            TOTAL_SUPPLY,
        );

        let metadata =
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name,
                symbol,
                icon,
                reference: None,
                reference_hash: None,
                decimals: TOKEN_DECIMALS,
            };
            metadata.assert_valid();

        Self {
            owner_id,
            creator_id,
            launch_vault_id,
            token,
            metadata,
            buy_tax_bps,
            sell_tax_bps,
            factory_contract_id,
        }
    }

    pub fn get_total_supply(&self) -> near_sdk::json_types::U128 {
        self.token.total_supply.into()
    }

    pub fn get_creator(&self) -> AccountId {
        self.creator_id.clone()
    }

    pub fn get_launch_vault(&self) -> AccountId {
        self.launch_vault_id.clone()
    }

    pub fn get_buy_tax_bps(&self) -> u16 {
        self.buy_tax_bps
    }

    pub fn get_sell_tax_bps(&self) -> u16 {
        self.sell_tax_bps
    }

    pub fn set_buy_tax_bps(
        &mut self,
        value: u16,
    ) {
        self.assert_owner();

        assert!(
            value <= MAX_TAX_BPS,
            "Buy tax cannot exceed 10%"
        );

        self.buy_tax_bps = value;
    }

    pub fn set_sell_tax_bps(
        &mut self,
        value: u16,
    ) {
        self.assert_owner();

        assert!(
            value <= MAX_TAX_BPS,
            "Sell tax cannot exceed 10%"
        );

        self.sell_tax_bps = value;
    }

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only token owner can perform this action"
        );
    }
}

#[near]
impl FungibleTokenCore for NearMeMePadToken {
    #[payable]
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: near_sdk::json_types::U128,
        memo: Option<String>,
    ) {
        self.token.ft_transfer(
            receiver_id,
            amount,
            memo,
        );
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: near_sdk::json_types::U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<near_sdk::json_types::U128> {
        self.token.ft_transfer_call(
            receiver_id,
            amount,
            memo,
            msg,
        )
    }

    fn ft_total_supply(
        &self,
    ) -> near_sdk::json_types::U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(
        &self,
        account_id: AccountId,
    ) -> near_sdk::json_types::U128 {
        self.token.ft_balance_of(account_id)
    }
}

#[near]
impl FungibleTokenResolver for NearMeMePadToken {
    #[private]
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: near_sdk::json_types::U128,
    ) -> near_sdk::json_types::U128 {
        let (used_amount, burned_amount) = self
            .token
            .internal_ft_resolve_transfer(
                &sender_id,
                receiver_id,
                amount,
            );

        if burned_amount > 0 {
            env::log_str(&format!(
                "Burned {} tokens during transfer resolution",
                burned_amount
            ));
        }

        used_amount.into()
    }
}

#[near]
impl StorageManagement for NearMeMePadToken {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> near_contract_standards::storage_management::StorageBalance {
        self.token.storage_deposit(
            account_id,
            registration_only,
        )
    }

    #[payable]
    fn storage_withdraw(
        &mut self,
        amount: Option<near_sdk::NearToken>,
    ) -> near_contract_standards::storage_management::StorageBalance {
        self.token.storage_withdraw(amount)
    }

    fn storage_unregister(
        &mut self,
        force: Option<bool>,
    ) -> bool {
        self.token.storage_unregister(force)
    }

    fn storage_balance_bounds(
        &self,
    ) -> near_contract_standards::storage_management::StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(
        &self,
        account_id: AccountId,
    ) -> Option<
        near_contract_standards::storage_management::StorageBalance
    > {
        self.token.storage_balance_of(account_id)
    }
}

#[near]
impl FungibleTokenMetadataProvider
    for NearMeMePadToken
{
    fn ft_metadata(
        &self,
    ) -> FungibleTokenMetadata {
        self.metadata.clone()
    }
}




// use near_contract_standards::fungible_token::{
//     FungibleToken,
//     FungibleTokenResolver,
//     FungibleTokenCore,
// };
// use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

// use near_contract_standards::storage_management::StorageManagement;

// use near_sdk::{
//     env,
//     near,
//     AccountId,
//     PanicOnDefault,
// };

// const TOTAL_SUPPLY: u128 =
//     1_000_000_000_000_000_000_000_000_000_000_000_000;

// const DECIMALS: u8 = 24;

// const MAX_TAX_BPS: u16 = 1_000;

// #[near(contract_state)]
// #[derive(PanicOnDefault)]
// pub struct NearMeMePadToken {
//     pub owner_id: AccountId,

//     pub creator_id: AccountId,

//     pub name: String,

//     pub symbol: String,

//     pub decimals: u8,

//     pub token: FungibleToken,

//     pub metadata: FungibleTokenMetadata,

//     pub buy_tax_bps: u16,

//     pub sell_tax_bps: u16,

//     pub treasury_contract_id: AccountId,

//     pub rewards_contract_id: AccountId,
// }

// #[near]
// impl NearMeMePadToken {
//     // =========================================================
//     // INITIALIZE TOKEN
//     // =========================================================

//     #[init]
//     pub fn new(
//         owner_id: AccountId,

//         creator_id: AccountId,

//         name: String,

//         symbol: String,

//         buy_tax_bps: u16,

//         sell_tax_bps: u16,

//         treasury_contract_id: AccountId,

//         rewards_contract_id: AccountId,

//         icon: Option<String>,

//         reference: Option<String>,

//         reference_hash: Option<near_sdk::json_types::Base64VecU8>,

//         spec: String,
//     ) -> Self {
//         assert!(
//             !env::state_exists(),
//             "Token is already initialized"
//         );

//         assert!(
//             !name.trim().is_empty(),
//             "Token name is required"
//         );

//         assert!(
//             !symbol.trim().is_empty(),
//             "Token symbol is required"
//         );

//         assert!(
//             buy_tax_bps <= MAX_TAX_BPS,
//             "Buy tax cannot exceed 10%"
//         );

//         assert!(
//             sell_tax_bps <= MAX_TAX_BPS,
//             "Sell tax cannot exceed 10%"
//         );

//         let mut token =
//             FungibleToken::new(b"t");

//         let metadata =
//             FungibleTokenMetadata {
//                 spec,

//                 name: name.clone(),

//                 symbol: symbol.clone(),

//                 icon,

//                 reference,

//                 reference_hash,

//                 decimals: DECIMALS,
//             };

//         token.internal_register_account(
//             &creator_id
//         );

//         token.internal_deposit(
//             &creator_id,
//             TOTAL_SUPPLY,
//         );

//         Self {
//             owner_id,

//             creator_id,

//             name,

//             symbol,

//             decimals: DECIMALS,

//             token,

//             metadata,

//             buy_tax_bps,

//             sell_tax_bps,

//             treasury_contract_id,

//             rewards_contract_id,
//         }
//     }

//     // =========================================================
//     // TOKEN METADATA
//     // =========================================================

//     pub fn get_token_info(
//         &self,
//     ) -> TokenInfo {
//         TokenInfo {
//             name: self.name.clone(),

//             symbol: self.symbol.clone(),

//             decimals: self.decimals,

//             total_supply: TOTAL_SUPPLY,

//             creator_id: self.creator_id.clone(),

//             buy_tax_bps: self.buy_tax_bps,

//             sell_tax_bps: self.sell_tax_bps,
//         }
//     }

//     // =========================================================
//     // TAX INFORMATION
//     // =========================================================

//     pub fn get_buy_tax_bps(
//         &self,
//     ) -> u16 {
//         self.buy_tax_bps
//     }

//     pub fn get_sell_tax_bps(
//         &self,
//     ) -> u16 {
//         self.sell_tax_bps
//     }

//     // =========================================================
//     // CREATOR
//     // =========================================================

//     pub fn get_creator(
//         &self,
//     ) -> AccountId {
//         self.creator_id.clone()
//     }

//     // =========================================================
//     // TOTAL SUPPLY
//     // =========================================================

//     pub fn get_total_supply(
//         &self,
//     ) -> near_sdk::json_types::U128 {
//         near_sdk::json_types::U128(
//             self.token.total_supply
//         )
//     }

//     // =========================================================
//     // UPDATE TAX
//     // =========================================================

//     pub fn set_taxes(
//         &mut self,

//         buy_tax_bps: u16,

//         sell_tax_bps: u16,
//     ) {
//         self.assert_owner();

//         assert!(
//             buy_tax_bps <= MAX_TAX_BPS,
//             "Buy tax cannot exceed 10%"
//         );

//         assert!(
//             sell_tax_bps <= MAX_TAX_BPS,
//             "Sell tax cannot exceed 10%"
//         );

//         self.buy_tax_bps = buy_tax_bps;

//         self.sell_tax_bps = sell_tax_bps;
//     }

//     // =========================================================
//     // OWNER
//     // =========================================================

//     pub fn set_owner(
//         &mut self,
//         owner_id: AccountId,
//     ) {
//         self.assert_owner();

//         self.owner_id = owner_id;
//     }

//     // =========================================================
//     // SECURITY
//     // =========================================================

//     fn assert_owner(&self) {
//         assert_eq!(
//             env::predecessor_account_id(),
//             self.owner_id,
//             "Only token owner can perform this action"
//         );
//     }
// }

// // =============================================================
// // TOKEN INFO
// // =============================================================

// #[near(serializers = [json])]
// #[derive(Clone)]
// pub struct TokenInfo {
//     pub name: String,

//     pub symbol: String,

//     pub decimals: u8,

//     pub total_supply: u128,

//     pub creator_id: AccountId,

//     pub buy_tax_bps: u16,

//     pub sell_tax_bps: u16,
// }

// // =============================================================
// // NEP-141 CORE
// // =============================================================

// #[near]
// impl FungibleTokenCore for NearMeMePadToken {
//     #[payable]
//     fn ft_transfer(
//         &mut self,
//         receiver_id: AccountId,
//         amount: near_sdk::json_types::U128,
//         memo: Option<String>,
//     ) {
//         self.token.ft_transfer(
//             receiver_id,
//             amount,
//             memo,
//         );
//     }

//     #[payable]
//     fn ft_transfer_call(
//         &mut self,
//         receiver_id: AccountId,
//         amount: near_sdk::json_types::U128,
//         memo: Option<String>,
//         msg: String,
//     ) -> near_sdk::PromiseOrValue<
//         near_sdk::json_types::U128
//     > {
//         self.token.ft_transfer_call(
//             receiver_id,
//             amount,
//             memo,
//             msg,
//         )
//     }

//     fn ft_total_supply(
//         &self,
//     ) -> near_sdk::json_types::U128 {
//         self.token.ft_total_supply()
//     }

//     fn ft_balance_of(
//         &self,
//         account_id: AccountId,
//     ) -> near_sdk::json_types::U128 {
//         self.token.ft_balance_of(account_id)
//     }
// }

// // =============================================================
// // NEP-141 RESOLVER
// // =============================================================

// #[near]
// impl FungibleTokenResolver for NearMeMePadToken {
//     #[private]
//     fn ft_resolve_transfer(
//         &mut self,

//         sender_id: AccountId,

//         receiver_id: AccountId,

//         amount: near_sdk::json_types::U128,
//     ) -> near_sdk::json_types::U128 {
//         self.token.ft_resolve_transfer(
//             sender_id,
//             receiver_id,
//             amount,
//         )
//     }
// }

// // =============================================================
// // STORAGE MANAGEMENT
// // =============================================================

// #[near]
// impl StorageManagement for NearMeMePadToken {
//     #[payable]
//     fn storage_deposit(
//         &mut self,

//         account_id: Option<AccountId>,

//         registration_only: Option<bool>,
//     ) -> near_contract_standards::storage_management::StorageBalance {
//         self.token.storage_deposit(
//             account_id,
//             registration_only,
//         )
//     }

//     #[payable]
//     fn storage_withdraw(
//         &mut self,

//         amount: Option<near_sdk::NearToken>,
//     ) -> near_contract_standards::storage_management::StorageBalance {
//         self.token.storage_withdraw(amount)
//     }

//     #[payable]
//     fn storage_unregister(
//         &mut self,

//         force: Option<bool>,
//     ) -> bool {
//         self.token.storage_unregister(force)
//     }

//     fn storage_balance_bounds(
//         &self,
//     ) -> near_contract_standards::storage_management::StorageBalanceBounds {
//         self.token.storage_balance_bounds()
//     }

//     fn storage_balance_of(
//         &self,

//         account_id: AccountId,
//     ) -> Option<
//         near_contract_standards::storage_management::StorageBalance
//     > {
//         self.token.storage_balance_of(account_id)
//     }
// }

// // =============================================================
// // NEP-148 METADATA
// // =============================================================

// #[near]
// impl near_contract_standards::fungible_token::metadata::FungibleTokenMetadataProvider
//     for NearMeMePadToken
// {
//     fn ft_metadata(
//         &self,
//     ) -> FungibleTokenMetadata {
//         self.metadata.clone()
//     }
// }

// pub const TOKEN_DECIMALS: u8 = 24;

// pub const TOTAL_SUPPLY: u128 =
//     1_000_000_000u128 * 10u128.pow(24);

//     fn should_graduate(
//     &self,
//     launch: &Launch,
// ) -> bool {
//     let curve = match &launch.bonding_curve {
//         Some(curve) => curve,
//         None => return false,
//     };

//     if curve.graduation_market_cap == 0 {
//         return false;
//     }

//     let quote_reserve =
//         curve.quote_reserve;

//     quote_reserve >= curve.graduation_market_cap
// }