pub struct Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<u32>> = Vec::with_capacity(9);
        let mut cols: Vec<HashSet<u32>> = Vec::with_capacity(9);
        let mut sub_box: Vec<HashSet<u32>> = Vec::with_capacity(9);
        for _ in 0..9 {
            rows.push(HashSet::new());
            cols.push(HashSet::new());
            sub_box.push(HashSet::new());
        }
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    if let Some(num) = board[i][j].to_digit(10) {
                        let box_index = (i / 3) * 3 + j / 3;
                        if rows[i].contains(&num) ||
                            cols[j].contains(&num) ||
                            sub_box[box_index].contains(&num) {
                                return false;
                            }
                        rows[i].insert(num);
                        cols[j].insert(num);
                        sub_box[box_index].insert(num);
                    }
                }
            }
        }
        true
    }
}
