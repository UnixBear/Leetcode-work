pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut currentReach = 0;
        let mut nextReach = 0;
        let destination = nums.len() as i32 - 1;
        if destination == 0 {
            return 0;
        }
        for (index, &value) in nums.iter().enumerate() {
            let index = index as i32;
            nextReach = std::cmp::max(nextReach, index + value);
            if index == currentReach {
                currentReach = nextReach;
                jumps += 1;
            }
            if currentReach >= destination {
                return jumps;
            }
        }
        return jumps;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}