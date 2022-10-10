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
    }
}
