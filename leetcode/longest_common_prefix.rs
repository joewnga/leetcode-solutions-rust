/// 
/// # Problem
/// 
/// Write a function to find the longest common prefix string amongst an array of strings.
/// 
/// If there is no common prefix, return an empty string "".
/// 
/// Example 1:
/// 
/// Input: strs = ["flower","flow","flight"]
/// Output: "fl"
/// 
/// 
/// # Solution 1:
/// Time complexity: 0(n * m)
/// Space complexity: 0(m)

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            while !s.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return "".to_string();
                }
            }
        }
        prefix
    }
}