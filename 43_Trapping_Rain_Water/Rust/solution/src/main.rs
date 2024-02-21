mod solution;
use solution::Solution;
fn main() {
    let test = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!("Hello world! We will test the set [0,1,0,2,1,0,1,3,2,1,2,1] and we should get 6");
    println!("The answer is: [{}]", Solution::trap(test));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }
}