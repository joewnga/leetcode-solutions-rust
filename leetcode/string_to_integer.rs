/// 
/// # Problem
/// 
/// Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
/// 
/// The algorithm for myAtoi(string s) is as follows:
/// 1. Read in and ignore any leading whitespace.
/// 2. Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
/// 3. Read in next the characters until the next non-digit charcter or the end of the input is reached. The rest of the string is ignored.
/// 4. Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
/// 5. If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
/// 6. Return the integer as the final result.
/// 
/// Note:
///    Only the space character ' ' is considered a whitespace character.
///   Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
/// 

/// # Example
/// 
/// ```
/// Input: s = "42"
/// Output: 42
/// 
/// Input: s = "   -42"
/// Output: -42


/// # Solution
/// Time complexity: O(n)
/// Space complexity: O(1)

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        let (s, sign) = match s.strip_prefix('-') {
            Some(s) => (s, -1),
            None => (s.strip_prefix('+').unwrap_or(s), 1),
        };
        s.chars()
            .map(|c| c.to_digit(10))
            .take_while(Option::is_some)
            .flatten()
            .fold(0, |acc, digit| {
                acc.saturating_mul(10).saturating_add(sign * digit as i32)
            })
    
        
    }
}