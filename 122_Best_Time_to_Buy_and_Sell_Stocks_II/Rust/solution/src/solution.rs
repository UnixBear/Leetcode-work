pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit : i32 = 0;
        for pair in prices.windows(2) {
            if let [current,next] = pair {
                if current < next {
                    profit += next - current;
                }
            }
        }
        profit
    }
}