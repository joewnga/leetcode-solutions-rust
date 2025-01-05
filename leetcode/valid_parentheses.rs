/// 
/// Problem: Valid Parentheses
/// 
/// Given a string `s` containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
/// 
/// A string is valid if:
/// 1. Open brackets must be closed by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
/// 
/// Example 1:
/// Input: s = "()"
/// Output: true
/// 
/// Example 2:
/// Input: s = "()[]{}"
/// Output: true
/// 
/// Example 3:
/// Input: s = "(]"
/// Output: false
/// 
/// # Solution:
///
/// Time complexity: O(1)
/// Space complexity: O(n)


impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}'|')'|']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }   
        return stack.is_empty();
    }
}
