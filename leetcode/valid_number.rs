/// 
/// Problem: Valid Number
/// 
/// A **valid number** can be an integer, a decimal, or a number in **scientific notation** (e.g., `"1e10"`).
///
/// Write a function that **determines if a given string is a valid number**.
///
/// **A valid number must follow these rules:**
/// - It may contain digits (`0-9`), a **sign (`+/-`)**, a **decimal (`.`)**, and an **exponent (`e` or `E`)**.
/// - A **decimal** number **must have digits on at least one side of `.`** (e.g., `"1.23"` is valid but `"."` is not).
/// - An **exponent** (`e` or `E`) **must be followed by an integer**.
/// - The number **must not contain any extra characters**.
///
/// **Example 1:**
/// ```plaintext
/// Input: s = "2"
/// Output: true
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s = "0.1"
/// Output: true
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: s = "abc"
/// Output: false
/// ```
///
/// **Example 4:**
/// ```plaintext
/// Input: s = "1e10"
/// Output: true
/// ```
///
/// **Constraints:**
/// - `1 <= s.length <= 20`
/// - `s` consists of English letters (upper/lowercase), digits, and symbols (`+`, `-`, `.`).
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut seen_digit = false;
        let mut seen_dot = false;
        let mut seen_exp = false;
        
        let chars: Vec<char> = s.chars().collect();
        
        for (i, &ch) in chars.iter().enumerate() {
            match ch {
                '0'..='9' => seen_digit = true,
                '+' | '-' => {
                    if i > 0 && chars[i - 1] != 'e' && chars[i - 1] != 'E' {
                        return false;
                    }
                }
                '.' => {
                    if seen_dot || seen_exp {
                        return false;
                    }
                    seen_dot = true;
                }
                'e' | 'E' => {
                    if seen_exp || !seen_digit {
                        return false;
                    }
                    seen_exp = true;
                    seen_digit = false; 
                }
                _ => return false,
            }
        }

        seen_digit
    }
}