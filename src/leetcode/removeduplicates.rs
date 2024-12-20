pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut start: usize = 1;
        let mut idx: usize = 0;

        while start <= (nums.len() - 1) {
            if nums[start] == nums[idx] {
                start += 1;
            } else {
                idx += 1;
                nums[idx] = nums[start];
                start += 1;
            }
        }
        (idx as i32) + 1
    }
}