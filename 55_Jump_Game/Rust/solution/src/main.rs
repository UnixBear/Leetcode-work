mod solution;
use solution::Solution;

fn main() {
    let test_one = vec!(2,3,1,1,4);
    let test_two = vec!(3,2,1,0,4);
    
    println!("The first test of (2,3,1,1,4) should return true: {}", Solution::can_jump(test_one));
    println!("The first test of (3,2,1,0,4) should return true: {}", Solution::can_jump(test_two));
}
