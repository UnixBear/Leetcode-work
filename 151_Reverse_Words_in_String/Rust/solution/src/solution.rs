pub struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let words : Vec<&str> = s.split_whitespace().collect();
        let reversed_words = words.into_iter().rev();
        let answer = reversed_words.collect::<Vec<&str>>().join(" ");
        answer
    }
}