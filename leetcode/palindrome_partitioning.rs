/// 
/// Problem: Palindrome Partitioning
/// 
/// Given a string s, partition s such that every substring of the partition is a palindrome.
/// Return all possible palindrome partitioning of s.
/// 
/// Example 1:
/// Input: s = "aab"
/// Output: [["a","a","b"],["aa","b"]]
/// 
/// Example 2:
/// Input: s = "a"
/// Output: [["a"]]
/// 
/// Constraints:
/// 1 <= s.length <= 16
/// s contains only lowercase English letters.
/// 

// # Solution
// Time complexity: O(N * 2^N) where N is the length of the string
// Space complexity: O(N^2)


impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
       let mut current_partition = Vec::new();
       let s_chars: Vec<char> = s.chars().collect();
       
       Self::backtrack(&s_chars, 0, &mut current_partition, &mut result);
       
       result
   }
   
   fn backtrack(s: &[char], start: usize, current: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
       if start >= s.len() {
           result.push(current.clone());
           return;
       }
       
       for end in start..s.len() {
           if Self::is_palindrome(s, start, end) {
               let substring: String = s[start..=end].iter().collect();
               current.push(substring);
               
               Self::backtrack(s, end + 1, current, result);
               
               current.pop();
           }
       }
   }
   
   fn is_palindrome(s: &[char], start: usize, end: usize) -> bool {
       let mut i = start;
       let mut j = end;
       
       while i < j {
           if s[i] != s[j] {
               return false;
           }
           i += 1;
           j -= 1;
       }
       
       true
    }
}