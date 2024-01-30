pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citation_list = citations;
        citation_list.sort_by(|a,b| b.cmp(a));
        let mut tracker = 0;
        for (index,citation) in citation_list.into_iter().enumerate() {
            match citation >= (index as i32) + 1 {
                true => tracker = (index as i32) + 1,
                false => break,
            }
        }
        return tracker;
    }
}