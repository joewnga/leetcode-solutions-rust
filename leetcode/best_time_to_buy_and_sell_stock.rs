/// 
/// Problem: Best Time to Buy and Sell Stock
/// 
/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// 
/// You want to maximize your profit by choosing a single day to buy one stock and choosing a 
/// different day in the future to sell that stock.
/// 
/// Return the maximum profit you can achieve from this transaction. If you cannot achieve any 
/// profit, return 0.
/// 
/// Example 1:
/// Input: prices = [7,1,5,3,6,4]
/// Output: 5
/// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
/// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
/// 
/// Example 2:
/// Input: prices = [7,6,4,3,1]
/// Output: 0
/// Explanation: In this case, no transactions are done and the max profit = 0.
/// 
/// Constraints:
/// 1 <= prices.length <= 10^5
/// 0 <= prices[i] <= 10^4
/// 


// # Solution
// Time complexity: O(n) 
// Space complexity: O(1) 

impl Solution {
   pub fn max_profit(prices: Vec<i32>) -> i32 {
       if prices.is_empty() {
           return 0;
       }
       
       let mut min_price = prices[0];
       let mut max_profit = 0;
       
       for price in prices.iter().skip(1) {
           // Update max profit if selling at current price is better
           max_profit = max_profit.max(price - min_price);
           
           // Update min price if current price is lower
           min_price = min_price.min(*price);
       }
       
       max_profit
   }
}