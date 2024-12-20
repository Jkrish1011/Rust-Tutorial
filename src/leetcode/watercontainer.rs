/*
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.

 

Example 1:


Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
Example 2:

Input: height = [1,1]
Output: 1
 

Constraints:

n == height.length
2 <= n <= 105
0 <= height[i] <= 104

*/

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut start: usize = 0;
        let mut end: usize = height.len() - 1;
        let mut result: i32 = 0;
        let mut minValue: i32 = 0;
        while start < end {
            // println!("start: {} | end: {} | height[start]: {} | height[end]: {}", start, end, height[start], height[end]);
            minValue = std::cmp::min(height[start], height[end]);
            
            result = std::cmp::max(minValue * (end - start) as i32, result);
            // println!("Result value: {}", result);

            if height[start] < height[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }
        
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_container(){
        let res: i32 = Solution::max_area(vec![1,8,6,2,5,4,8,3,7]);
        assert_eq!(res, 49);
    }
}