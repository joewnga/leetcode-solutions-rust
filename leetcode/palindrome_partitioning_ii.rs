/// 
/// Problem: Palindrome Partitioning II
/// 
/// Given a string s, partition s such that every substring of the partition is a palindrome.
/// 
/// Return the minimum cuts needed for a palindrome partitioning of s.
/// 
/// Example 1:
/// Input: s = "aab"
/// Output: 1
/// Explanation: The palindrome partitioning ["aa","b"] could be produced using 1 cut.
/// 
/// Example 2:
/// Input: s = "a"
/// Output: 0
/// 
/// Example 3:
/// Input: s = "ab"
/// Output: 1
/// 
/// Constraints:
/// 1 <= s.length <= 2000
/// s consists of lowercase English letters only.
/// 

// # Solution 
// Time complexity: O(n^2) where n is the length of the string
// Space complexity: O(n)

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
       let n = chars.len();
       
       let mut cuts = vec![0; n];
       for i in 0..n {
           cuts[i] = i;
       }
       
       for center in 0..n {
           let mut left = center as i32;
           let mut right = center;
           
           while left >= 0 && right < n && chars[left as usize] == chars[right] {
               if left == 0 {
                   cuts[right] = 0;
               } else {
                   cuts[right] = cuts[right].min(cuts[left as usize - 1] + 1);
               }
               
               left -= 1;
               right += 1;
           }
           
           left = center as i32;
           right = center + 1;
           
           while left >= 0 && right < n && chars[left as usize] == chars[right] {
               if left == 0 {
                   cuts[right] = 0;
               } else {
                   cuts[right] = cuts[right].min(cuts[left as usize - 1] + 1);
               }
               
               
               left -= 1;
               right += 1;
           }
       }
       
       cuts[n - 1] as i32
    }
}