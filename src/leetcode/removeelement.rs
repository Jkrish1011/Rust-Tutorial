pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        // let mut k = 0;
        // for idx in 0..nums.len() {
        //     if nums[idx] != val {
        //         nums[k] = nums[idx];
        //         k+=1;
        //     }
        // }
        // return k as i32;

        let mut start: i32 = 0;
        let mut end: i32 = (nums.len() - 1) as i32;
        println!("start::{}", start);
        println!("end::{}", end);
        while start <= end {
            println!("nums[start]::{}", nums[start as usize]);
            if nums[start as usize] == val {
                println!("start::{}", start);
                nums[start as usize] = nums[end as usize];
                end-=1;
            } else{
                println!("end::{}", end);
                start+=1;
            }
        }
        return (end+1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        // let mut v = vec![3,2,2,3];
        let mut v = vec![1];
        let s = Solution::remove_element(&mut v, 1);
        println!("{:?}", s);
    }
}