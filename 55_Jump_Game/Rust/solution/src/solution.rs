pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut maxReach = nums[0];
        let destination = (nums.len() - 1) as i32;
        for i in 1i32..destination {
            if (i > maxReach) {
                return false;
            }
            maxReach = maxReach.max(i + nums[i as usize]);
            if (maxReach>=destination) {
                return true;
            }
        }
        return maxReach >= destination;
    }
}