# INSTRUCTIONS

Given an integer array `data` representing the data, return whether it is a valid **UTF-8** encoding (i.e. it translates to a sequence of valid UTF-8 encoded characters).

A character in **UTF8** can be from **1 to 4 bytes** long, subjected to the following rules:

1. For a **1-byte** character, the first bit is a 0, followed by its Unicode code.
2. For an **n-bytes** character, the first n bits are all one's, the n + 1 bit is 0, followed by n - 1 bytes with the most significant 2 bits being 10.

This is how the UTF-8 encoding would work:



     Number of Bytes   |        UTF-8 Octet Sequence
                       |              (binary)
     ------------------+-----------------------------------------
            1          |   0xxxxxxx
            2          |   110xxxxx 10xxxxxx
            3          |   1110xxxx 10xxxxxx 10xxxxxx
            4          |   11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

x denotes a bit in the binary form of a byte that may be either 0 or 1.

Note: The input is an array of integers. Only the least significant 8 bits of each integer is used to store the data. This means each integer represents only 1 byte of data.

 

Example 1:

```
Input: data = [197,130,1]
Output: true
Explanation: data represents the octet sequence: 11000101 10000010 00000001.
It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte character.
```
Example 2:
```
Input: data = [235,140,4]
Output: false
Explanation: data represented the octet sequence: 11101011 10001100 00000100.
The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes character.
The next byte is a continuation byte which starts with 10 and that's correct.
But the second continuation byte does not start with 10, so it is invalid.
 ```

Constraints:

- `1 <= data.length <= 2 * 104`
- `0 <= data[i] <= 255`

# PRELIMINARY THOUGHTS
Luckily, we're restricted to 4 bytes lengths of characters so we don't have to worry about the n-bound much.  I'm less familiar with javascript than I should be, so in this case I'll be using JS for my solution.  I don't have to provide a translation function so I won't have any isomorphic considerations.  

step 0) learn js's int->binary methods and other bitwise functions
     note: javascript represents negative numbers in two's-competent notation but luckily by constraint all data numbers are strictly positive

step 1) ~~map out which numbers fall within each byte range.~~ Scratch that, better idea: shift bits over to see if shifted number matches any of our states.  Example: if I want to check `197 (11000101)`, i could first see if, shifted right by `5(110)`, it equals 110 which can only be the two byte setup for `110xxxxx 10xxxxxx`. I could then have have a switch statement to check through the 4 by incremental shifts

### Todo list
[ ] mapping

~~step 1a) figure out switch statements in js or just a long if statement~~

step 1 mapping)

if `(n>>>5).toString(2)`, which is how javascript gives us out byte formats, gives us `110`, we have 2 bytes: `110xxxxx 10xxxxxx` 
if `(n>>>4).toString(2)` gives us 1110, we have 3 bytes: `1110xxxx 10xxxxxx 10xxxxxx`
if `(n>>>3).toString(2)` gives us 11110, we have 4 bytes:`11110xxx 10xxxxxx 10xxxxxx 10xxxxxx`

step 2) We need to figure out how to keep track of our bytes; ~~my first intuition is a while or for loop with an iterator variable.~~ I need a tracker variable that initializes at 0 and stays 0 if the numbers make it through the switches.  

 With the helper function done, we can do a direct comparison on the array now that it's byte converted.

step 3) I've come up with a seeminly efficient way to do this, in the O(1) territory.
     It will at most loop 3 times with at most 5 checks, which means at most we have O(15) aka O(1)
1. I have a tracking variable that returns 1 of 3 numbers if certain conditions are met, and by the end if it's not 0 then it is not valid.  
2. the first loop will check for the first state, which is the initial sequence. Since the tracking veriable starts at 0, and the 0 condition checks if the sequence represents the 2-byte state, we check if that's the case. If it is, then we set the tracker to 1.
3. if the tracker is 1, then either we've reached the end and it's not a valid octet sequence or we move to the next number.  The only way to decrement the tracker is if the tracking variable not 0 and the sequence starts with 10 because anything else would invalidate the octet.  Getting to this part presupposes we're not at the first byte.  

[x] helper functions

~~translating numbers into full bytes~~

`convertUTF` via mapping converts into byte length
     



Credit to wingkwong for his C++ solution for the inspiration on the remainder idea.
