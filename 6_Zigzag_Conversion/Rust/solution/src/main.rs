mod solution;
use solution::Solution;
fn main() {
    println!("run cargo test")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());

    }
}

