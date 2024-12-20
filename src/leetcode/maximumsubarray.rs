/*
Given an integer array nums, find the subarray with the largest sum, and return its sum.

Example 1:
----------
Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: The subarray [4,-1,2,1] has the largest sum 6.

Example 2:
----------
Input: nums = [1]
Output: 1
Explanation: The subarray [1] has the largest sum 1.

Example 3:
----------
Input: nums = [5,4,-1,7,8]
Output: 23
Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
 

Constraints:

1 <= nums.length <= 105
-104 <= nums[i] <= 104
 

Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

   max_so_far = arr[0]
    max_ending_here = arr[0]

    for i = 1 to length(arr) - 1:
        max_ending_here = max(arr[i], max_ending_here + arr[i])
        max_so_far = max(max_so_far, max_ending_here)

    return max_so_far


*/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_ending_here: i32 = nums[0];
        let mut max_so_far: i32 = nums[0];

        for idx in 1..=nums.len() - 1 {
            max_ending_here = std::cmp::max(nums[idx], max_ending_here + nums[idx]);
            max_so_far = std::cmp::max(max_so_far, max_ending_here);
        }

        max_so_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_maximum_subarray() {
        let res = Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]);
        assert_eq!(res, 6);
    }
}