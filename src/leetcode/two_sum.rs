pub struct Solution;
use std::collections::HashMap;

impl Solution {
    fn TwoSum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&complement_idx) = hash.get(&complement) {
                return vec![complement_idx as i32, i as i32]
            }
            hash.insert(*num, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn testSolution() {
        assert_eq!(Solution::TwoSum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}