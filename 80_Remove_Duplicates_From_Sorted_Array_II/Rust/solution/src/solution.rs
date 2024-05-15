pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        let mut insert_pos = 2;
        for i in 2..nums.len() {
            if nums[i] != nums[insert_pos - 2] {
                nums[insert_pos] = nums[i];
                insert_pos += 1;
            }
        }

        insert_pos as i32
    }
}
