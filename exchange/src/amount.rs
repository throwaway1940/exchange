use rust_decimal::Decimal;

/// Monetary amounts are represented as decimals interally.
/// This prevents rounding and conversion issues.
/// We use `rust_decimal` as it integrates well with `serde`
/// and is a well-tested crate
pub type Amount = Decimal;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_amount_normal() {
        let amount = Amount::from_str("1.2345").unwrap();
        assert_eq!(format!("{}", amount), "1.2345");
    }

    #[test]
    fn test_amount_truncated() {
        let amount = Amount::from_str("1.23456789").unwrap();
        assert_eq!(format!("{}", amount.round_dp(4)), "1.2346");
    }
}
