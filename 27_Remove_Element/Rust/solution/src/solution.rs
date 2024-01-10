pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let temp: Vec<i32> = nums.iter().filter(|&&x| x != val).cloned().collect();
        nums.clear();
        nums.extend(temp);
        return nums.len() as i32;
    }
    pub fn remove_element2(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }
}
