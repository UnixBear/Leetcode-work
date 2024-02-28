pub struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut answer = String::new();
        let numerals = [(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];
        for &(value,symbol) in &numerals{
            while num >=value {
                num -= value;
                answer.push_str(symbol)
            }

        }
        answer
        
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_numbers() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(58), "LVIII");

        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}