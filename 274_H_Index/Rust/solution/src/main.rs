mod solution;

use solution::Solution;
fn main() {
    let testvec = vec![3,0,6,1,5];
    println!("{}",Solution::h_index(testvec));
}
