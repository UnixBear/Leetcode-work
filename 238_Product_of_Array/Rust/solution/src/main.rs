pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut product = 1;
        let mut answer : Vec<i32> = vec![];
        let mut zeroes = 0;
        for num in &nums {
            if *num != 0 {
                product = product * num;
            }
            else {
                zeroes += 1;
            }
        }
        for num in nums {
            if zeroes > 1 {
            answer.push(0);
        }
        else if zeroes == 1{
            if num == 0{
                answer.push(product)
            }
            if num != 0{
                answer.push(0)
            }
        }
        else{
            answer.push(product / num)
        }

        }
        answer
    }
}

fn main() {
    let test1 = vec![1,2,3,4];
    let test2 = vec![-1,1,0,-3,3];
    println!("Hello, world!, we will test [1,2,3,4] and [-1,1,0,-3,3], which should yield [24,12,8,6] and [0,0,9,0,0] respectively");
    assert_eq!(Solution::product_except_self(test1), vec![24,12,8,6], "expected [24,12,8,6]");
    assert_eq!(Solution::product_except_self(test2), vec![0,0,9,0,0], "expected [0,0,9,0,0] ");
    println!("all tests passed");

}
