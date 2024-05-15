mod solution;
use solution::Solution;
fn main() {
    let mut nums:Vec<i32> = vec![2,2,1,1,1,2,2];
    println!("the answer is: {}", Solution::majority_element(nums));
}
