/*
Given an integer x, return true if x is a palindrome, and false otherwise.

Example 1:
Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:
Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 
*/
pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut revNumber = 0;
        let mut num = x;
        while num != 0 {
            let tmp = num % 10;
            num = num / 10;
            revNumber = 10 * revNumber + tmp;
        }
        if revNumber == x {
            return true;
        }
        else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn TestPalindrome(){
        let res = Solution::is_palindrome(10);
        println!("result: {}", res);
    }
}