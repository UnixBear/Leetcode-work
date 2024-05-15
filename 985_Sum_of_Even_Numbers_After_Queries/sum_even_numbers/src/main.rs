

pub struct Solution {
    rtn_val:i32,
}

//finished solution
/* 
impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        //this creates the initial sum that will be changing as we go through queries and pushing to our
        //residual vector
        let mut sum : i32 = nums.iter().filter(|num| **num % 2 == 0).sum();

        //this will be where we store the finished vector that will be the set of sums
        let mut answer : Vec<i32> = vec![];

        //loop through queries and for each loop, copy values of query for comparison.
        //each loop we subtract the value given by nums[index] from the queryfrom the running 
        //sum if it's divisible by 2. this is because if its not already even then it was never
        //added to the sum in the first place.  then perform the regular operation of adding 
        //the value in the query to the nums vector value and then check if it's divisible by
        //2. if it is, add it to the running sum and then push the sum into the answer vector
        for query in queries {
            let index = query[0] as usize;
            let value = query[1];
            if nums[index] % 2 == 0 {sum -= nums[index]}
            nums[index] += value;
            if nums[index] % 2 == 0 {sum += nums[index]}
            answer.push(sum);
        }
        answer
    }
}
*/

/*
impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        //this creates the initial sum that will be changing as we go through queries and pushing to our
        //residual vector
        let mut sum: i32 = nums.iter().filter(|num| **num % 2 == 0).sum();

        //this will be where we store the finished vector that will be the set of sums
        let mut rez: Vec<i32> = vec![];

        
        //this takes an iterator of our queries and returns another iterator where each
        //query is now a slice of the query vector so we can access the elements like arrays
        //this avoids any manual index access requirements
        let mut it = queries.iter().map(|query| query.as_slice());

        //here we start a while loop to iterate through it, which contains itemized queries as
        //slices and make the truth condition be whether it has another element in it
        while let Some(&[val, index]) = it.next() {
            let index = index as usize;
            if nums[index] % 2 == 0 { sum -= nums[index]}
            nums[index] += val;
            if nums[index] % 2 == 0 { sum += nums[index]}
            rez.push(sum);
        }
        rez
    }
}
*/

impl Solution {
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        
        queries.iter().filter_map(|query| if let &[val, index] = query.as_slice() { 
            Some((val, index as usize)) } else { None })
            .scan(nums.iter().filter(|num| **num % 2 == 0).sum::<i32>(), |sum, (val, index)| {
                if nums[index] % 2 == 0 { *sum -= nums[index]}
                nums[index] += val;
                if nums[index] % 2 == 0 { *sum += nums[index]}
                Some(*sum)
            }).collect()
    }
}


fn main() {
    let x = 10 as f64;
    assert_eq!(x, 10.0);
}




