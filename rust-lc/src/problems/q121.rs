use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut min = i32::MAX;
        let mut max_profit = 0;


        for price in prices {
            min = min.min(price);
            max_profit = max_profit.max(price - min);
        }

        max_profit
    }
}