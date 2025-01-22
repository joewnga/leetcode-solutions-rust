/// 
/// Problem: Group Anagrams
/// 
/// Given an array of strings `strs`, group the anagrams together.
///
/// **Example 1:**
/// ```plaintext
/// Input: strs = ["eat","tea","tan","ate","nat","bat"]
/// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: strs = [""]
/// Output: [[""]]
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: strs = ["a"]
/// Output: [["a"]]
/// ```
///
/// **Constraints:**
/// - `1 <= strs.length <= 10^4`
/// - `0 <= strs[i].length <= 100`
/// - `strs[i]` consists of lowercase English letters.
///
/// # Solution:
///
/// - **Time Complexity:** `O(n * k log k)`
/// - **Space Complexity:** `O(n)`
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut key = s.chars().collect::<Vec<char>>();
            key.sort_unstable();
            let key = key.iter().collect::<String>();

            map.entry(key).or_insert(Vec::new()).push(s);
        }

        map.into_values().collect()
        
    }
}