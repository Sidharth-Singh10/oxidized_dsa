//https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = 999999;
        let mut profit = 0;
        for &price in &prices {
            if price < min_price {
                min_price = price;
            } else {
                profit = price - min_price;
            }
            if profit > max_profit {
                max_profit = profit;
            } 
        }
        max_profit
    }
}
