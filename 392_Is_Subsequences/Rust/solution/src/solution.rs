pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t_iterator = t.chars(); // create iterator of t characters
        s.chars().all(|ch| t_iterator.any(|t_ch| t_ch == ch)) 
        //return true when ch in s.chars passed to the iterator's all method for equivalence
    }
}