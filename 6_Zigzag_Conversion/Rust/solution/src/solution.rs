pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || num_rows >= s.len() as i32 {
            return s;
        }
        let mut answer : Vec<String> = vec![String::from(""); num_rows as usize];
        let mut current_line = 0;
        let mut edge_reached = true;
        
        for char in s.chars() {
            if (current_line == 0 || current_line == (num_rows-1)) {
                edge_reached = !edge_reached;
            }
            answer[current_line as usize].push(char);
            if !edge_reached {
                current_line += 1;
            }
            else {
                current_line -= 1;
            }
        }
        answer.iter().fold(String::new(), |acc, s| acc + s)
    }
}
