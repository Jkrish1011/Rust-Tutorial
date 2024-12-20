pub struct Solution;

impl Solution {
    pub fn generate_contiguous_subarray(arr: Vec<i32>) {
        
        for i in 0..arr.len() {
            let mut tmp: Vec<i32> = vec![];
            for j in i..arr.len() {
                // tmp.push(arr[i..j+1]);
                println!("{:?}", &arr[i..j+1]);
            }
            // println!("{:?}", &tmp);
        }

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_contiguous_subarray() {
        Solution::generate_contiguous_subarray(vec![1,1,2]);
        let r = 2 | 0;
        println!("{}", r);
    }
}