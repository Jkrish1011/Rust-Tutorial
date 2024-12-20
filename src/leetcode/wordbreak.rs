/*
Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.
Note that the same word in the dictionary may be reused multiple times in the segmentation.

--------- 

Example 1:

Input: s = "leetcode", wordDict = ["leet","code"]
Output: true
Explanation: Return true because "leetcode" can be segmented as "leet code".

--------- 

Example 2:

Input: s = "applepenapple", wordDict = ["apple","pen"]
Output: true
Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
Note that you are allowed to reuse a dictionary word.

--------- 

Example 3:

Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
Output: false
 
--------- 

Constraints:

1 <= s.length <= 300
1 <= wordDict.length <= 1000
1 <= wordDict[i].length <= 20
s and wordDict[i] consist of only lowercase English letters.
All the strings of wordDict are unique.

*/

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let length: usize = s.len();
        let mut dp: Vec<bool> = vec![false; length + 1];
        dp[length] = true;
        
        

        for idx in (0..=length-1).rev() {
            
            for w in &word_dict {
                
                if (idx + w.len()) <= length && w == &s[idx..idx+w.len()] {
                    
                    dp[idx] = dp[idx + w.len()];
                }
                if dp[idx] == true {
                    break;
                }
            }
        }
        
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        let res: bool = Solution::word_break(String::from("catsandog"), vec!["cats".to_string(), "dog".to_string()]);
        assert_eq!(res, false);
    }
}