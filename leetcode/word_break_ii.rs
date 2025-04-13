/// 
/// Problem: Word Break II
/// 
/// Given a string s and a dictionary of strings wordDict, add spaces in s to construct a 
/// sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
/// 
/// Note that the same word in the dictionary may be reused multiple times in the segmentation.
/// 
/// Example 1:
/// Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
/// Output: ["cats and dog","cat sand dog"]
/// 
/// Example 2:
/// Input: s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
/// Output: ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
/// 
/// Example 3:
/// Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
/// Output: []
/// 
/// Constraints:
/// 1 <= s.length <= 20
/// 1 <= wordDict.length <= 1000
/// 1 <= wordDict[i].length <= 10
/// s and wordDict[i] consist of only lowercase English letters.
/// All the strings of wordDict are unique.
/// 

// # Solution
// Time complexity: O(n * 2^n) 
// Space complexity: O(n * 2^n)

use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
       let mut memo: HashMap<usize, Vec<String>> = HashMap::new();
       
       Self::dfs(&s, 0, &word_set, &mut memo)
   }
   
   fn dfs(s: &str, start: usize, word_set: &HashSet<String>, memo: &mut HashMap<usize, Vec<String>>) -> Vec<String> {
       if let Some(result) = memo.get(&start) {
           return result.clone();
       }
       
       let mut result = Vec::new();
       
       if start == s.len() {
           result.push(String::new());
           return result;
       }
       
       for end in start + 1..=s.len() {
           let prefix = &s[start..end];
           
           if word_set.contains(prefix) {
               let sub_segments = Self::dfs(s, end, word_set, memo);
               
               for segment in sub_segments {
                   if segment.is_empty() {
                       result.push(prefix.to_string());
                   } else {
                       result.push(format!("{} {}", prefix, segment));
                   }
               }
           }
       }
       
       memo.insert(start, result.clone());
       result
    }
}