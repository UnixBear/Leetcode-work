mod solution;

use solution::Solution;
fn main() {
    println!("Hello, world!");
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    println!("We will test the vectors gas:[1,2,3,4,5] and cost:[3,4,5,1,2]. The answer should be 3 \n");
    println!("solution returned: [{}]", Solution::can_complete_circuit(gas,cost));
}
