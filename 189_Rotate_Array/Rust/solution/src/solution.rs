pub struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let rot_am = k % nums.len() as i32;
        let axis = nums.len() as i32 - rot_am;
        let mut temp = nums.split_off(axis as usize);
        temp.append(nums);
        *nums = temp;

    }
}