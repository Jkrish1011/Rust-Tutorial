/*
Find the Index of the First Occurrence in a String

Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 
if needle is not part of haystack.

Example 1:
----------
Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Example 2:
----------
Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.

*/

pub struct Solution;

impl Solution {
    pub fn str_str2(haystack: String, needle: String) -> i32 {
        let mut s_needle: Vec<char> = needle.chars().collect();
        let mut s_haystack: Vec<char> = haystack.chars().collect();
        let mut idx_haystack: i32 = 0;
        let mut idx_needle: usize = 0;
        // println!("{}", &s_needle.nth(1).unwrap());
        for (index, c) in haystack.chars().enumerate() {
            println!("needle_char:: {}", &s_needle[idx_needle]);
            println!("haystack_char:: {}", &c);
            println!("idx_needle:: {}", &idx_needle);
            
            if c == s_needle[idx_needle] {
                println!("incrementing idx_needle");
                idx_needle+=1;
                if idx_needle == needle.len() {
                    println!("index:: {}", &index);
                    println!("idx_needle:: {}", &idx_needle);
                    return (index as i32 - idx_needle as i32 + 1) as i32;
                }
            } else {
                println!("resetting idx_needle");
                idx_needle = 0;
            }
            println!("idx_needle::end {}", &idx_needle);
        }
        -1
    }
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let mut start_index_vector: Vec<usize> = vec![];
        let mut s_needle: Vec<char> = needle.chars().collect();
        let mut s_haystack: Vec<char> = haystack.chars().collect();
        let mut idx_haystack: usize = 0;

        while idx_haystack < haystack.len() {
            if s_haystack[idx_haystack] == s_needle[0] {
                start_index_vector.push(idx_haystack);
            }
            idx_haystack+=1;
        }
        idx_haystack=0;
        
        for start_index in start_index_vector {
            
            if start_index + s_needle.len() > s_haystack.len() {
                continue;
            }
            idx_haystack = start_index.clone();
            let mut found_flag: bool = true;
            for idx in 0..s_needle.len() {
                if s_haystack[idx_haystack] == s_needle[idx] {
                    idx_haystack+=1;
                } else {
                    idx_haystack = 0;
                    found_flag = false;
                    break;
                }
            }
            if found_flag {
                return start_index as i32;
            }
            
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_need_in_haystack() {
        let res = Solution::str_str(String::from("leetcode"), String::from("but"));
        assert_eq!(res, -1);
    }
}