mod solution;
use solution::Solution;
fn main() {
    let mut numbers = vec![1,3,3,5,6];
    let testnum = 3;
    Solution::remove_element(&mut numbers, testnum);
    println!("testing the vector [1,3,3,5,6] and removing 3");
    println!("{:?}", numbers);
    numbers = vec![1,3,3,5,6];
    Solution::remove_element2(&mut numbers, testnum);
    println!("testing with retain the vector [1,3,3,5,6] and removing 3");
    println!("{:?}", numbers);
}
