/// 
/// Problem: Valid Palindrome
/// 
/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters 
/// and removing all non-alphanumeric characters, it reads the same forward and backward.
/// Alphanumeric characters include letters and numbers.
/// 
/// Given a string s, return true if it is a palindrome, or false otherwise.
/// 
/// Example 1:
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
/// 
/// Example 2:
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
/// 
/// Example 3:
/// Input: s = " "
/// Output: true
/// Explanation: s is an empty string "" after removing non-alphanumeric characters.
/// Since an empty string reads the same forward and backward, it is a palindrome.
/// 
/// Constraints:
/// 1 <= s.length <= 2 * 10^5
/// s consists only of printable ASCII characters.
/// 

// # Solution 
// Time complexity: O(n)
// Space complexity: O(1) 

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
       let mut left = 0;
       let mut right = chars.len() as i32 - 1;
       
       while left < right {
           while left < right && !chars[left as usize].is_alphanumeric() {
               left += 1;
           }
           
           while left < right && !chars[right as usize].is_alphanumeric() {
               right -= 1;
           }
           
           if chars[left as usize].to_ascii_lowercase() != chars[right as usize].to_ascii_lowercase() {
               return false;
           }
           
           left += 1;
           right -= 1;
       }
       
       true
    }
}