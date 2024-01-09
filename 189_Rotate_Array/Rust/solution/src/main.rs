mod solution;
use solution::Solution;

fn main() {
    println!("Vector used is: [1,2,3,4,5] with rotation by 2 \n and output should be [4,5,1,2,3]");
    let mut test = vec![1,2,3,4,5];
    let k = 2;
    Solution::rotate(&mut test, k);
    println!("The rotated vector is: {:?}", &test);
    
}
