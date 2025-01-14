/// 
/// Problem: Substring with Concatenation of All Words
/// 
/// You are given a string `s` and an array of strings `words`. All the strings in `words` are of the same length.
/// A concatenated substring in `s` is a substring that is a concatenation of each word in `words` exactly once, in any order, 
/// and without any intervening characters.
///
/// Return the starting indices of all such substrings in `s`. You can return the answer in any order.
///
/// Example 1:
/// Input: s = "barfoothefoobarman", words = ["foo","bar"]
/// Output: [0,9]
/// Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar", respectively.
///
/// Example 2:
/// Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
/// Output: []
///
/// Example 3:
/// Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
/// Output: [6,9,12]
///
/// Constraints:
/// - `1 <= s.length <= 10^4`
/// - `1 <= words.length <= 5000`
/// - `1 <= words[i].length <= 30`
///
/// Time Complexity: O(n * m), where:
/// - `n` is the length of `s`.
/// - `m` is the number of words.
///
/// 
/// Space Complexity: O(m)
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return Vec::new();
        }

        let single_word_length = words[0].len();
        let concatenated_length = words.len() * single_word_length;

        if s.len() < concatenated_length {
            return Vec::new();
        }

        let word_frequencies = words.into_iter().fold(HashMap::new(), |mut frequencies, word| {
            *frequencies.entry(word).or_insert(0) += 1;
            frequencies
        });

        (0..=s.len() - concatenated_length)
            .filter_map(|start_index| {
                let mut remaining = word_frequencies.clone();
                let mut current_position = start_index;

                while current_position < start_index + concatenated_length {
                    let substring = &s[current_position..current_position + single_word_length];

                    if let Some(count) = remaining.get_mut(substring) {
                        *count -= 1;
                        if *count == 0 {
                            remaining.remove(substring);
                        }
                    } else {
                        return None;
                    }

                    current_position += single_word_length;
                }

                Some(start_index as i32)
            })
            .collect()
    }
}
