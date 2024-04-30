mod solution;
use solution::Solution;
fn main() {
    let s = "abc";
    let t = "ahbgdc";
    let answer = Solution::is_subsequence(s.to_string(), t.to_string());
    println!("{}", answer);
}
