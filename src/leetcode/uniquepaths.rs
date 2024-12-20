/*
There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). 
The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The test cases are generated so that the answer will be less than or equal to 2 * 109.

Example 1:
Input: m = 3, n = 7
Output: 28

Example 2:
Input: m = 3, n = 2
Output: 3

Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Down -> Down
2. Down -> Down -> Right
3. Down -> Right -> Down
 

Constraints:

1 <= m, n <= 100
*/

pub struct Solution;

impl Solution {
    fn create_ones(length: usize) -> Vec<i32> {
        vec![1; length]
    }
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let length: usize = n as usize;
        let mut row = Solution::create_ones(length);
        let mut newRow = Solution::create_ones(length);
        
        for rowIdx in (0..=m-2).rev() {
            // println!("Current Row: {}", rowIdx.clone());
            for idx in (0..=n-2).rev() {
                // println!("Current Column: {}", idx.clone());
                newRow[idx as usize] = newRow[idx as usize +1] + row[idx as usize];
            } 
            // println!("New Row is : {:?}", newRow);
            row = newRow.clone();
        }
        row[0]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn TestUniquePaths() {
        let res: i32 = Solution::unique_paths(3 as i32, 7 as i32);
        assert_eq!(res, 28);
    }
}