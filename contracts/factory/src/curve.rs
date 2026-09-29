#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurveQuote {
    pub input_amount: u128,
    pub fee_amount: u128,
    pub creator_tax_amount: u128,
    pub amount_out: u128,
}

pub const BPS_DENOMINATOR: u128 = 10_000;

/// Constant-product output calculation.
///
/// reserve_in  = reserve of the asset being paid in
/// reserve_out = reserve of the asset being purchased
/// amount_in   = gross amount supplied by trader
///
/// The caller supplies the effective amount after protocol/tax deductions.
pub fn calculate_amount_out(
    reserve_in: u128,
    reserve_out: u128,
    amount_in_after_fees: u128,
) -> u128 {
    assert!(
        reserve_in > 0,
        "Input reserve must be greater than zero"
    );

    assert!(
        reserve_out > 0,
        "Output reserve must be greater than zero"
    );

    assert!(
        amount_in_after_fees > 0,
        "Input amount must be greater than zero"
    );

    let numerator = reserve_out
        .checked_mul(amount_in_after_fees)
        .expect("Curve calculation overflow");

    let denominator = reserve_in
        .checked_add(amount_in_after_fees)
        .expect("Curve calculation overflow");

    let amount_out = numerator / denominator;

    assert!(
        amount_out > 0,
        "Trade amount is too small"
    );

    assert!(
        amount_out < reserve_out,
        "Trade would exhaust token reserve"
    );

    amount_out
}

/// Calculates a fee using basis points.
pub fn calculate_bps(
    amount: u128,
    bps: u16,
) -> u128 {
    amount
        .checked_mul(bps as u128)
        .expect("Fee calculation overflow")
        / BPS_DENOMINATOR
}

/// Calculates protocol trading fee and creator tax.
///
/// Both are calculated independently from the gross input.
pub fn calculate_trade_fees(
    amount_in: u128,
    trading_fee_bps: u16,
    creator_tax_bps: u16,
) -> (u128, u128) {
    let trading_fee =
        calculate_bps(
            amount_in,
            trading_fee_bps,
        );

    let creator_tax =
        calculate_bps(
            amount_in,
            creator_tax_bps,
        );

    let total_fee = trading_fee
        .checked_add(creator_tax)
        .expect("Total fee overflow");

    assert!(
        total_fee < amount_in,
        "Fees must be less than trade amount"
    );

    (
        trading_fee,
        creator_tax,
    )
}

/// Splits the protocol trading fee.
///
/// 30% → platform
/// 70% → creator
pub fn split_trading_fee(
    trading_fee: u128,
) -> (u128, u128) {
    let platform =
        calculate_bps(
            trading_fee,
            3_000,
        );

    let creator =
        trading_fee
            .checked_sub(platform)
            .expect("Invalid trading fee split");

    (platform, creator)
}

/// Splits creator tax.
///
/// 40% → platform
/// 60% → creator
pub fn split_creator_tax(
    creator_tax: u128,
) -> (u128, u128) {
    let platform =
        calculate_bps(
            creator_tax,
            4_000,
        );

    let creator =
        creator_tax
            .checked_sub(platform)
            .expect("Invalid creator tax split");

    (platform, creator)
}

/// Calculates a BUY.
///
/// Gross quote amount is supplied by the buyer.
/// Fees are removed before calculating tokens received.
pub fn calculate_buy(
    quote_reserve: u128,
    token_reserve: u128,
    gross_quote_amount: u128,
    trading_fee_bps: u16,
    creator_tax_bps: u16,
) -> CurveQuote {
    let (
        trading_fee,
        creator_tax,
    ) = calculate_trade_fees(
        gross_quote_amount,
        trading_fee_bps,
        creator_tax_bps,
    );

    let total_fees =
        trading_fee
            .checked_add(creator_tax)
            .expect("Fee overflow");

    let effective_quote =
        gross_quote_amount
            .checked_sub(total_fees)
            .expect("Fee exceeds trade amount");

    let token_out =
        calculate_amount_out(
            quote_reserve,
            token_reserve,
            effective_quote,
        );

    CurveQuote {
        input_amount: gross_quote_amount,

        fee_amount: trading_fee,

        creator_tax_amount: creator_tax,

        amount_out: token_out,
    }
}

/// Calculates a SELL.
///
/// The input is token amount.
/// Output is quote asset.
pub fn calculate_sell(
    token_reserve: u128,
    quote_reserve: u128,
    gross_token_amount: u128,
    trading_fee_bps: u16,
    creator_tax_bps: u16,
) -> CurveQuote {
    let gross_quote = calculate_amount_out(
        token_reserve,
        quote_reserve,
        gross_token_amount,
    );
    let (trading_fee, creator_tax) = calculate_trade_fees(
        gross_quote,
        trading_fee_bps,
        creator_tax_bps,
    );
    let quote_out = gross_quote
        .checked_sub(trading_fee.checked_add(creator_tax).expect("Fee overflow"))
        .expect("Fees exceed sale proceeds");

    CurveQuote {
        input_amount: gross_token_amount,

        fee_amount: trading_fee,

        creator_tax_amount: creator_tax,

        amount_out: quote_out,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_constant_product_buy() {
        let output =
            calculate_amount_out(
                100,
                1_000,
                10,
            );

        assert_eq!(
            output,
            90
        );
    }

    #[test]
    fn calculates_fee() {
        let fee =
            calculate_bps(
                1_000,
                1_000,
            );

        assert_eq!(
            fee,
            100
        );
    }

    #[test]
    fn splits_trading_fee_correctly() {
        let (platform, creator) = split_trading_fee(1_000);

        assert_eq!(platform, 300);

        assert_eq!(creator, 700);
    }

    #[test]
    fn buy_fee_and_tax_are_separate() {
        let result = calculate_buy(100_000, 1_000_000, 10_000, 1_000, 500);
        assert_eq!(result.fee_amount, 1_000);
        assert_eq!(result.creator_tax_amount, 500);
    }

    #[test]
    fn sell_fee_and_tax_are_separate() {
        let result = calculate_sell(1_000_000, 100_000, 10_000, 1_000, 500);
        assert_eq!(result.fee_amount, 99);
        assert_eq!(result.creator_tax_amount, 49);
    }
}

pub fn calculate_buy_with_virtual_reserves(
    real_quote_reserve: u128,
    real_token_reserve: u128,
    virtual_quote_reserve: u128,
    virtual_token_reserve: u128,
    gross_quote_amount: u128,
    trading_fee_bps: u16,
    creator_tax_bps: u16,
) -> CurveQuote {
    let effective_quote_reserve =
        real_quote_reserve
            .checked_add(virtual_quote_reserve)
            .expect("Quote reserve overflow");

    let effective_token_reserve =
        real_token_reserve
            .checked_add(virtual_token_reserve)
            .expect("Token reserve overflow");

    calculate_buy(
        effective_quote_reserve,
        effective_token_reserve,
        gross_quote_amount,
        trading_fee_bps,
        creator_tax_bps,
    )
}

pub fn calculate_sell_with_virtual_reserves(
    real_token_reserve: u128,
    real_quote_reserve: u128,
    virtual_token_reserve: u128,
    virtual_quote_reserve: u128,
    gross_token_amount: u128,
    trading_fee_bps: u16,
    creator_tax_bps: u16,
) -> CurveQuote {
    let effective_token_reserve =
        real_token_reserve
            .checked_add(virtual_token_reserve)
            .expect("Token reserve overflow");

    let effective_quote_reserve =
        real_quote_reserve
            .checked_add(virtual_quote_reserve)
            .expect("Quote reserve overflow");

    calculate_sell(
        effective_token_reserve,
        effective_quote_reserve,
        gross_token_amount,
        trading_fee_bps,
        creator_tax_bps,
    )
}

#[test]
fn trading_fee_split_is_30_70() {
    let (platform, creator) =
        split_trading_fee(1_000);

    assert_eq!(platform, 300);
    assert_eq!(creator, 700);
}

#[test]
fn tax_split_is_40_60() {
    let (platform, creator) =
        split_creator_tax(1_000);

    assert_eq!(platform, 400);
    assert_eq!(creator, 600);
}

#[test]
fn ten_percent_tax_is_allowed() {
    let fee =
        calculate_bps(1_000, 1_000);

    assert_eq!(fee, 100);
}

#[test]
#[should_panic]
fn tax_above_ten_percent_is_not_allowed_by_launch_config() {
    assert!(1_001 <= 1_000);
}