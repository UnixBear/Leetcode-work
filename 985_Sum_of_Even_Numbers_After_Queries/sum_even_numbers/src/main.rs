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
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
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
    }
}*/


fn main() {
    let testvec = vec![1,2,3,4];
    let mut testsum : i32 = testvec.into_iter().sum();
    println!("{}", &testsum)
    //let evenvec = testvec.into_iter().filter(|x| x % 2 == 0).collect::<Vec<i32>>();
    //for element in &evenvec {println!("{}", element);}
}




