mod solution;
use solution::Solution;

fn main() {
    println!("run cargo test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(numbers, target), vec![1,2]);
    }
    #[test]
    fn test_no_solution() {
        let numbers = vec![1, 2, 3, 4, 5];
        let target = 100;
        assert_eq!(Solution::two_sum(numbers, target), vec![]);
    }

    #[test]
    fn test_negative_numbers() {
        let numbers = vec![-3, 1, 2, 4, 5];
        let target = 1;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 5]);
    }

    #[test]
    fn test_large_array() {
        let numbers = vec![1; 1000];
        let target = 2;
        assert_eq!(Solution::two_sum(numbers, target), vec![1, 2]);
    }
}

