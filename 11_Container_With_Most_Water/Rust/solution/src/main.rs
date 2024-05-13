mod solution;
use solution::Solution;

fn main() {
    println!("run `cargo test`");
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_max_are() {
        let test_height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(test_height);
        assert_eq!(result, 49);
    }

    fn test_empty() {
        let test_height = vec![];
        let result = Solution::max_area(test_height);
        assert_eq!(result, 0)
    }

    fn test_small() {
        let test_height = vec![1, 1];
        let result = Solution::max_area(test_height);
        assert_eq!(result,1);
    }

}
