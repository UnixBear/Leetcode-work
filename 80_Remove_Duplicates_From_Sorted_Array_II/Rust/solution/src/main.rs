mod solution;
use solution::Solution;


fn main() {
    let mut nums = vec![0,0,1,1,1,1,2,3,3];
    let k = Solution::remove_duplicates(&mut nums);
    println!("The length of the array after removing duplicates is: {}", k);
    println!("The first {} elements of the modified array are: {:?}", k, &nums[..k as usize]);
}
