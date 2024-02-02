pub struct Solution;

impl Solution {
    pub fn can_jump(nums: &[i32]) -> bool {
        let mut max_reach = nums[0];
        let destination = (nums.len() - 1) as i32;
        for i in 1i32..destination {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + nums[i as usize]);
            if max_reach>=destination {
                return true;
            }
        }
        return max_reach >= destination;
    }
    pub fn can_jump2(nums: &[i32]) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mut max_reach = 0usize;
        let destination = nums.len() - 1;
        
        for (i, &jump_length) in nums.iter().enumerate().take(destination) {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i+jump_length as usize);
            if max_reach >= destination {
                return true;
            }
        }
        return max_reach >= destination;
    }
}