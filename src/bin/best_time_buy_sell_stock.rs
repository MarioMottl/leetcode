use std::cmp::{max, min};

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for price in prices {
        min_price = min(min_price, price);
        max_profit = max(max_profit, price - min_price);
    }

    max_profit
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profitable() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5)
    }

    #[test]
    fn test_non_profitable() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0)
    }
}
