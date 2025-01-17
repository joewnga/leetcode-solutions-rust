/// 
/// Problem: Longest Valid Parentheses
/// 
/// Given a string containing just the characters `'('` and `')'`, find the length of the longest valid (well-formed) parentheses substring.
///
/// Example 1:
/// Input: s = "(()"
/// Output: 2
/// Explanation: The longest valid parentheses substring is "()".
///
/// Example 2:
/// Input: s = ")()())"
/// Output: 4
/// Explanation: The longest valid parentheses substring is "()()".
///
/// Example 3:
/// Input: s = ""
/// Output: 0
///
/// Constraints:
/// - `0 <= s.length <= 10^4`
/// - `s` consists of only `'('` and `')'`.
///
/// # Solution:
/// (O(n) time, O(n) space)
/// 
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut max_len = 0;

        for (i, ch) in s.chars().enumerate() {
            if ch == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if let Some(&last) = stack.last() {
                    max_len = max_len.max(i as i32 - last);
                } else {
                    stack.push(i as i32);
                }
            }
        }

        max_len
        
    }
}
