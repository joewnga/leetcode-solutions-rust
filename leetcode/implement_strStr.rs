/// 
/// Problem: Find the Index of the First Occurrence in a String
/// 
/// Given two strings `needle` and `haystack`, return the index of the first occurrence of `needle` in `haystack`, 
/// or `-1` if `needle` is not part of `haystack`.
///
/// Example 1:
/// Input: haystack = "sadbutsad", needle = "sad"
/// Output: 0
/// Explanation: "sad" occurs at index 0 and 6. The first occurrence is at index 0.
///
/// Example 2:
/// Input: haystack = "leetcode", needle = "leeto"
/// Output: -1
/// Explanation: "leeto" did not occur in "leetcode".
///
/// Constraints:
/// - 1 <= haystack.length, needle.length <= 10^4
/// - `haystack` and `needle` consist of only lowercase English characters.
///
/// # Solution:
/// A sliding window or substring comparison approach can solve this problem efficiently.
/// 
/// Time complexity: O(n * m)
/// Space complexity: O(1) 
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_len = haystack.len();
        let needle_len = needle.len();

        if needle_len == 0 {
            return 0; 
        }
        if needle_len > haystack_len {
            return -1; 
        }

        for i in 0..=(haystack_len - needle_len) {
            if &haystack[i..i + needle_len] == needle {
                return i as i32; 
            }
        }

        -1 
    }
}
