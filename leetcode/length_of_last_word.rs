/// 
/// Problem: Length of Last Word
/// 
/// Given a string `s` consisting of words and spaces, return the **length of the last word** in the string.
/// A word is defined as a **maximal substring consisting of non-space characters only**.
///
/// **Example 1:**
/// ```plaintext
/// Input: s = "Hello World"
/// Output: 5
/// Explanation: The last word is "World" with length 5.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s = "   fly me   to   the moon  "
/// Output: 4
/// Explanation: The last word is "moon" with length 4.
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: s = "luffy is still joyboy"
/// Output: 6
/// Explanation: The last word is "joyboy" with length 6.
/// ```
///
/// **Constraints:**
/// - `1 <= s.length <= 10^4`
/// - `s` consists of only English letters and spaces.
/// - There will be **at least one word** in `s`.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim()
            .split_whitespace()
            .last()
            .unwrap()
            .len() as i32
    }
}