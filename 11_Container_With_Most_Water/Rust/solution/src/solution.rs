pub struct Solution;
use std::cmp::min;
use std::cmp::max;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height == vec![] {
            return 0;
        }
        let mut left:usize = 0;
        let mut right = height.len() - 1;
        let mut maxVol = 0;
        while (left < right) {
            let vol = (right - left) as i32 * min(height[right], height[left]);
            maxVol = maxVol.max(vol);
            if height[right] < height[left] {
                right -= 1;
            }
            else {
                left += 1;
            }

        }
        maxVol
    }
}