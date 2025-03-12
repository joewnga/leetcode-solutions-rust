/// 
/// Problem: Distinct Subsequences
/// 
/// Given two strings s and t, return the number of distinct subsequences of s which equals t.
/// 
/// A subsequence of a string is a new string which is formed from the original string by 
/// deleting some (can be none) of the characters without disturbing the relative positions 
/// of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
/// 
/// The test cases are generated so that the answer fits on a 32-bit signed integer.
/// 
/// Example 1:
/// Input: s = "rabbbit", t = "rabbit"
/// Output: 3
/// Explanation:
/// As shown below, there are 3 ways you can get "rabbit" from "rabbbit".
/// rabbbit
/// rabbbit
/// rabbbit
/// 
/// Example 2:
/// Input: s = "babgbag", t = "bag"
/// Output: 5
/// Explanation:
/// As shown below, there are 5 ways you can get "bag" from "babgbag".
/// babgbag
/// babgbag
/// babgbag
/// babgbag
/// babgbag
/// 
/// Constraints:
/// 1 <= s.length, t.length <= 1000
/// s and t consist of English letters.
/// 

// # Solution: Recursive approach with memoization
// Time complexity: O(m*n) 
// Space complexity: O(m*n)

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
       let t_chars: Vec<char> = t.chars().collect();
       
       
       let mut memo = vec![vec![-1; t.len() + 1]; s.len() + 1];
       
       Self::count_subsequences(&s_chars, &t_chars, 0, 0, &mut memo) as i32
   }
   
   fn count_subsequences(
       s: &[char], 
       t: &[char], 
       s_index: usize, 
       t_index: usize, 
       memo: &mut Vec<Vec<i32>>
   ) -> i32 {
       
       if t_index == t.len() {
           return 1;
       }
       
       if s_index == s.len() {
           return 0;
       }
       
       if memo[s_index][t_index] != -1 {
           return memo[s_index][t_index];
       }
       
       let mut count = 0;
       
       if s[s_index] == t[t_index] {
           count += Self::count_subsequences(s, t, s_index + 1, t_index + 1, memo);
       }
       
       count += Self::count_subsequences(s, t, s_index + 1, t_index, memo);
       
       memo[s_index][t_index] = count;
       
       count
    }
}