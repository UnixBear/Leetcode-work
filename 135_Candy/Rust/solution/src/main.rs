mod solution;
use solution::Solution;
fn main() {
    let vec_one = vec![1,0,2];
    let vec_two = vec![1,2,2];
    println!("Hello World! We'll test the candy function on two vectors [1,0,2] and [1,2,2] and their respective results should be 5 and 4. \n");
    println!("Results: [{}] [{}]",Solution::candy(vec_one), Solution::candy(vec_two));

}
