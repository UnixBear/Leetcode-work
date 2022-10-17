use std::io;
use rand::{Rng, thread_rng};
use std::cmp::Ordering;
pub struct Solution {
    rtn_val:i32,
}
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        unimplemented!();
    }
}
/* 
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        //we want to get a 
        //let mut new_nums = nums.iter().filter(|num| **num%2 == 0).sum();
        let mut sum: i32 = nums.iter().filter(|num| **num % 2 == 0).sum();
        let mut rez = vec![];
        let mut it = queries.iter().map(|query| query.as_slice());
        while let Some(&[val, index]) = it.next() {
            let index = index as usize;
            if nums[index] % 2 == 0 { sum -= nums[index]}
            nums[index] += val;
            if nums[index] % 2 == 0 { sum += nums[index]}
            rez.push(sum);
        }
        rez
        return new_nums;
    }
}
*/
fn main() {
    println!("fkn guess");
    let mut sekrit_gen = thread_rng();
    let sekrit_num :u32 = sekrit_gen.gen_range(0..100);
    println!("psst the answer is {}", sekrit_num);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("u fkd up");
        let guess :u32 = guess.trim().parse().expect("dude this isn't a fkn number try again >;|");

         println!("u guessed: {}", guess);
        //println!("now we cumpair hehe: {}", guess.cmp(&sekrit_num))
        match guess.cmp(&sekrit_num) {
            Ordering::Less => println!("go higher, fuckface"),
            Ordering::Greater => println!("go lower, fuckface"),
            Ordering::Equal => {
                println!("go fuck yourself");
                break;
            }
            
        }
    }}





