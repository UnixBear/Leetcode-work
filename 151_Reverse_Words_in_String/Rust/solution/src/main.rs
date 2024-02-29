mod solution;
use solution::Solution;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_words("the sky is blue".to_string()), "blue is sky the".to_string());
        assert_eq!(Solution::reverse_words("  hello world  ".to_string()), "world hello".to_string());
        assert_eq!(Solution::reverse_words("a good   example".to_string()), "example good a".to_string());

    }
}