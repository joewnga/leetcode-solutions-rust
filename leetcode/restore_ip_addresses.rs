/// 
/// Problem: Restore IP Addresses
/// 
/// Given a string `s` containing **only digits**, return **all possible valid IP addresses** that can be formed by inserting dots (`.`).
/// 
/// A valid IP address consists of **four octets** (0-255), and no segment can have **leading zeros** unless it's "0".
///
/// **Example 1:**
/// ```plaintext
/// Input: s = "25525511135"
/// Output: ["255.255.11.135", "255.255.111.35"]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: s = "0000"
/// Output: ["0.0.0.0"]
/// ```
///
/// **Constraints:**
/// - `1 <= s.length <= 20`
/// - `s` consists of digits only.
///
/// # Solution:
/// - **Time Complexity:** `O(1)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut segments = Vec::new();
        Self::backtrack(&s, 0, &mut segments, &mut result);
        result
    }

    fn backtrack(s: &str, start: usize, segments: &mut Vec<String>, result: &mut Vec<String>) {
        if segments.len() == 4 && start == s.len() {
            result.push(segments.join("."));
            return;
        }

        if segments.len() >= 4 {
            return;
        }

        for length in 1..=3 {
            if start + length > s.len() {
                break;
            }

            let segment = &s[start..start + length];

            
            if (segment.starts_with('0') && segment.len() > 1) || 
               segment.parse::<u8>().map_or(true, |num| num > 255) {
                continue;
            }

            segments.push(segment.to_string());
            Self::backtrack(s, start + length, segments, result);
            segments.pop();
        }
    }
}