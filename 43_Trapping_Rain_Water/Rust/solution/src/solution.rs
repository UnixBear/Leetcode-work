pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (mut leftptr, mut rightptr, mut answer) = (0, height.len()-1,0);
        let (mut max_left, mut max_right) = (0,0);
        while leftptr < rightptr {
            if height[leftptr] < height[rightptr] {
                if height[leftptr] >= max_left {
                    max_left = height[leftptr];
                }
                else {
                    answer += max_left - height[leftptr];
                }
                leftptr += 1;
            }
            else {
                if height[rightptr] >= max_right {
                    max_right = height[rightptr];
                }
                else {
                    answer += max_right - height[rightptr];
                }
                rightptr -= 1;
            }
        }
        answer
        
    }
}