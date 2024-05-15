mod solution;
use solution::Solution;
fn main() {
    let test_string = "abcdefedcba";
    let sol = Solution::is_palindrome(test_string.to_string());
    println!("{}", sol);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindome() {
        let test1 = "A man, a plan, a canal: Panama";
        let test2 = "race a car";
        let test3 = " ";
        assert_eq!(Solution::is_palindrome(test1.to_string()), true);
        assert_eq!(Solution::is_palindrome(test2.to_string()), false);
        assert_eq!(Solution::is_palindrome(test3.to_string()), true);
    }
}