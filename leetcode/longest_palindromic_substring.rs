/// 
/// Problem: Longest Palindromic Substring
/// 
/// Given a string s, return the longest palindromic substring in s.
/// 
/// Example 1:
/// 
/// Input: s = "babad"
/// Output: "bab"
/// Note: "aba" is also a valid answer.
/// 

/// Solution 1:
/// Time complexity: O(n^3)
/// Space complexity: O(1)
/// 
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        
        let mut string_length = s.len();
        while string_length > 0 {
             match s.as_bytes()
                    .windows(string_length)
                    .find(|slice| { let iter = slice.iter();
                                        iter.clone().eq(iter.clone().rev()) 
                                }
                        ) {
                             Some(slice) => return String::from_utf8(slice.to_vec()).unwrap_or("".to_string()),
                             None => string_length -= 1, 
                        }
        }
        "".to_string() 
    }
}