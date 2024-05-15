pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map : HashMap<i32,i32> = HashMap::new();
        for num in nums {
            match map.get(&num) {
                Some(x) => { map.insert(num, x+1);}
                None => { map.insert(num, 1);}
            }
        }
        for (key,value) in &map {
            println!("{},{}", key,value);
        }
        *map.iter().max_by_key(|x| x.1).unwrap().0
    }
}  