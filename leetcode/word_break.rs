/// 
/// Problem: Word Break
/// 
/// Given a string s and a dictionary of strings wordDict, return true if s can be segmented 
/// into a space-separated sequence of one or more dictionary words.
/// 
/// Note that the same word in the dictionary may be reused multiple times in the segmentation.
/// 
/// Example 1:
/// Input: s = "leetcode", wordDict = ["leet","code"]
/// Output: true
/// Explanation: Return true because "leetcode" can be segmented as "leet code".
/// 
/// Example 2:
/// Input: s = "applepenapple", wordDict = ["apple","pen"]
/// Output: true
/// Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
/// Note that you are allowed to reuse a dictionary word.
/// 
/// Example 3:
/// Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
/// Output: false
/// 
/// Constraints:
/// 1 <= s.length <= 300
/// 1 <= wordDict.length <= 1000
/// 1 <= wordDict[i].length <= 20
/// s and wordDict[i] consist of only lowercase English letters.
/// All the strings of wordDict are unique.
/// 

// # Solution 1: Dynamic Programming
// Time complexity: O(n * m * k) where n is the length of s, m is the size of wordDict, and k is the average word length
// Space complexity: O(n) for the DP array

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
       
       let word_set: HashSet<String> = word_dict.into_iter().collect();
       
       let mut dp = vec![false; n + 1];
       dp[0] = true; 
       
       for i in 1..=n {
           for j in 0..i {
               
               if dp[j] && word_set.contains(&s[j..i].to_string()) {
                   dp[i] = true;
                   break;
               }
           }
       }
       
       dp[n]
    }
}