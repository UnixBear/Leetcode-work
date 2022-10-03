## Question
### 985. Sum of Even Numbers After Queries

You are given an integer array nums and an array queries where `queries[i] = [val_i, index_i]`.

For each query `i`, first, apply `nums[index_i] = nums[index_i] + val_i`, then print the sum of the even values of nums.

Return an *integer array* `answer` where `answer[i]` is the answer to the `ith` query.

    Example 1:

    Input: nums = [1,2,3,4], queries = [[1,0],[-3,1],[-4,0],[2,3]]
    Output: [8,6,2,4]
    Explanation: At the beginning, the array is [1,2,3,4].
    After adding 1 to nums[0], the array is [2,2,3,4], and the sum of even values is 2 + 2 + 4 = 8.
    After adding -3 to nums[1], the array is [2,-1,3,4], and the sum of even values is 2 + 4 = 6.
    After adding -4 to nums[0], the array is [-2,-1,3,4], and the sum of even values is -2 + 4 = 2.
    After adding 2 to nums[3], the array is [-2,-1,3,6], and the sum of even values is -2 + 6 = 4.


    Example 2:

    Input: nums = [1], queries = [[4,0]]
    Output: [0]


    Constraints:

        1 <= nums.length <= 104
        -104 <= nums[i] <= 104
        1 <= queries.length <= 104
        -104 <= vali <= 104
        0 <= indexi < nums.length

## Introduction

For this problem, what's it's really asking you is: "Can ya keep track of them array addresses, kiddo?"  I'm going to try and use Rust for this one because it's been a skill I've been meaning to hone up for a while.  

## Plan of attack

My first instinct is to, on recieving the array create another array `evenTracker` where `evenTracker.len()` = `nums.len()` and every element of `evenTracker` is `true`.

After that, I'll loop through `nums`and set its counterpart in `evenTracker` to false where the number is odd.

I have to design an add function for `queries` and if the result is odd or even, it will make the change to `evenTracker` accordingly

Finally, I'll look through `evenTracker` and for every `true` I will have a variable track the sum through `nums`


Let's see if this is a good idea!

## Notes
1. I discovered [this](https://cglab.ca/%7Eabeinges/blah/too-many-lists/book/README.html) tutorial using linked lists and I hope to use it more.

## Conclusion
TBD

## Todo list
    [] see if using a Vec versus an array would be better for performance