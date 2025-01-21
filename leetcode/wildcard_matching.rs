/// 
/// Problem: Wildcard Matching
/// 
/// Given an input string `s` and a pattern `p`, implement wildcard pattern matching with support for:
/// - `'?'` Matches **any** single character.
/// - `'*'` Matches **any sequence** of characters (including the empty sequence).
///
/// **Example 1:**
/// ```plaintext
/// Input: s = "aa", p = "a"
/// Output: false
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s = "aa", p = "*"
/// Output: true
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: s = "cb", p = "?a"
/// Output: false
/// ```
///
/// **Constraints:**
/// - `0 <= s.length, p.length <= 2000`
/// - `s` contains only lowercase English letters.
/// - `p` contains only lowercase English letters, '?' or '*'.
///
/// # Solution:
///
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (mut i, mut j) = (0, 0);
        let (mut star_idx, mut match_idx) = (-1, 0);
        let s = s.as_bytes();
        let p = p.as_bytes();

        while i < s.len() {
            if j < p.len() && (p[j] == b'?' || s[i] == p[j]) {
                i += 1;
                j += 1;
            } else if j < p.len() && p[j] == b'*' {
                star_idx = j as isize;
                match_idx = i;
                j += 1;
            } else if star_idx != -1 {
                j = (star_idx + 1) as usize;
                match_idx += 1;
                i = match_idx;
            } else {
                return false;
            }
        }

        while j < p.len() && p[j] == b'*' {
            j += 1;
        }

        j == p.len()
    }
}