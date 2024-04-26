pub struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        if chars.is_empty() {
            return true
        }
        let mut start: usize = 0;
        let mut end = chars.len() - 1;
        while start < end {
            if chars[start] != chars[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}
