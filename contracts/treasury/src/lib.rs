use near_sdk::{
    env,
    near,
    AccountId,
    NearToken,
    PanicOnDefault,
    Promise,
};

//const BASIS_POINTS: u128 = 10_000;

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Treasury {
    /// Account allowed to manage protocol configuration.
    pub owner_id: AccountId,

    /// Factory contract allowed to deposit protocol fees.
    pub factory_id: AccountId,

    /// Current protocol launch fee.
    pub launch_fee: NearToken,

    /// Platform share of protocol trading fee.
    pub trading_fee_platform_bps: u16,

    /// Platform share of creator buy/sell tax.
    pub trading_tax_platform_bps: u16,

    /// Total launch fees received.
    pub total_launch_fees: NearToken,

    /// Total trading fees received by platform.
    pub total_trading_fees: NearToken,

    /// Total creator-tax share received by platform.
    pub total_trading_taxes: NearToken,
}

#[near]
impl Treasury {
    #[init]
    pub fn new(
        owner_id: AccountId,
        factory_id: AccountId,
        launch_fee_yocto: u128,
    ) -> Self {
        assert!(
            !env::state_exists(),
            "Treasury is already initialized"
        );

        Self {
            owner_id,
            factory_id,

            launch_fee:
                NearToken::from_yoctonear(
                    launch_fee_yocto
                ),

            trading_fee_platform_bps: 3_000,

            trading_tax_platform_bps: 4_000,

            total_launch_fees:
                NearToken::from_yoctonear(0),

            total_trading_fees:
                NearToken::from_yoctonear(0),

            total_trading_taxes:
                NearToken::from_yoctonear(0),
        }
    }

    // --------------------------------------------------
    // VIEW METHODS
    // --------------------------------------------------

    pub fn get_launch_fee(&self) -> NearToken {
        self.launch_fee
    }

    pub fn get_trading_fee_platform_bps(&self) -> u16 {
        self.trading_fee_platform_bps
    }

    pub fn get_trading_tax_platform_bps(&self) -> u16 {
        self.trading_tax_platform_bps
    }

    pub fn get_total_launch_fees(&self) -> NearToken {
        self.total_launch_fees
    }

    pub fn get_total_trading_fees(&self) -> NearToken {
        self.total_trading_fees
    }

    pub fn get_total_trading_taxes(&self) -> NearToken {
        self.total_trading_taxes
    }

    pub fn get_balance(&self) -> NearToken {
        env::account_balance()
    }

    // --------------------------------------------------
    // FEE DEPOSITS
    // --------------------------------------------------

    /// Receives the mandatory 0.2 NEAR launch fee.
    ///
    /// Only the Factory can call this method.
    #[payable]
    pub fn receive_launch_fee(&mut self) {
        self.assert_factory();

        let amount = env::attached_deposit();

        assert!(
            amount == self.launch_fee,
            "Incorrect launch fee"
        );

        self.total_launch_fees =
            self.total_launch_fees.saturating_add(amount);
    }

    /// Receives the platform's 30% share
    /// of protocol trading fees.
    #[payable]
    pub fn receive_trading_fee(&mut self) {
        self.assert_factory();

        let amount = env::attached_deposit();

        assert!(
            amount > NearToken::from_yoctonear(0),
            "Trading fee must be greater than zero"
        );

        self.total_trading_fees =
            self.total_trading_fees.saturating_add(amount);
    }

    /// Receives the platform's 40% share
    /// of creator-configured buy/sell taxes.
    #[payable]
    pub fn receive_trading_tax(&mut self) {
        self.assert_factory();

        let amount = env::attached_deposit();

        assert!(
            amount > NearToken::from_yoctonear(0),
            "Trading tax must be greater than zero"
        );

        self.total_trading_taxes =
            self.total_trading_taxes.saturating_add(amount);
    }

    // --------------------------------------------------
    // ADMIN
    // --------------------------------------------------

    /// Change the launch fee.
    ///
    /// This should eventually be controlled by
    /// governance/multisig rather than a single owner.
    pub fn set_launch_fee(
        &mut self,
        launch_fee_yocto: u128,
    ) {
        self.assert_owner();

        assert!(
            launch_fee_yocto > 0,
            "Launch fee must be greater than zero"
        );

        self.launch_fee =
            NearToken::from_yoctonear(
                launch_fee_yocto
            );
    }

    /// Change the Factory contract.
    pub fn set_factory(
        &mut self,
        factory_id: AccountId,
    ) {
        self.assert_owner();

        self.factory_id = factory_id;
    }

    /// Withdraw platform funds.
    ///
    /// In production this should eventually be
    /// governed by a multisig/DAO.
    pub fn withdraw(
        &mut self,
        receiver_id: AccountId,
        amount_yocto: u128,
    ) -> Promise {
        self.assert_owner();

        let amount =
            NearToken::from_yoctonear(
                amount_yocto
            );

        assert!(
            amount <= env::account_balance(),
            "Insufficient treasury balance"
        );

        Promise::new(receiver_id)
            .transfer(amount)
    }

    // --------------------------------------------------
    // INTERNAL AUTHORIZATION
    // --------------------------------------------------

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only treasury owner can perform this action"
        );
    }

    fn assert_factory(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.factory_id,
            "Only the Factory can call this method"
        );
    }
}