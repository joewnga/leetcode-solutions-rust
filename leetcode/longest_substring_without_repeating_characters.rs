/// 
/// Problem: Longest Substring Without Repeating Characters
/// 
/// Given a string s, find the length of the longest substring without repeating characters.
/// 
///     Example 1:
/// 
///    Input: s = "abcabcbb"
///   Output: 3
///  Explanation: The answer is "abc", with the length of 3.
/// 
/// Optimal Solution:
/// Time complexity: O(n)
/// Space complexity: 0(min(n, m)) 

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
         let mut map = HashMap::new();
    let mut max_len = 0;
    let mut start = 0;

    for (i, c) in s.chars().enumerate() {
        
        if let Some(prev_index) = map.get(&c) {
            if *prev_index >= start {
                start = *prev_index + 1; 
            }
        }

        map.insert(c, i);
        max_len = max_len.max(i - start + 1); 
    }

    
    max_len as i32
    }
}



/// Brute Force Approach:
/// Time complexity: 0(n^2)
/// Space complexity: O(n)

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();  
        let n = chars.len();
        let mut max_len = 0;

        for i in 0..n {
            let mut seen = HashSet::new();
            for j in i..n {
                if seen.contains(&chars[j]) {
                    break;  
                }
                seen.insert(chars[j]);
                max_len = max_len.max(j - i + 1);
            }
        }

        max_len as i32
    }
}
