mod solution;
use solution::Solution;

fn main() {
    let test = vec![1,2,3,4,5];
    let answer = Solution::max_profit(test);
    println!("The correct answer should be 4.  The result is: {}", answer)
}
