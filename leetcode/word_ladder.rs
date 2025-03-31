/// 
/// Problem: Word Ladder
/// 
/// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence 
/// of words beginWord -> s1 -> s2 -> ... -> sk such that:
/// 
/// - Every adjacent pair of words differs by a single letter.
/// - Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
/// - sk == endWord
/// 
/// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the 
/// shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
/// 
/// Example 1:
/// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
/// Output: 5
/// Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> "cog", which is 5 words long.
/// 
/// Example 2:
/// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
/// Output: 0
/// Explanation: The endWord "cog" is not in wordList, so there is no valid transformation sequence.
/// 
/// Constraints:
/// 1 <= beginWord.length <= 10
/// endWord.length == beginWord.length
/// 1 <= wordList.length <= 5000
/// wordList[i].length == beginWord.length
/// beginWord, endWord, and wordList[i] consist of lowercase English letters.
/// beginWord != endWord
/// All the words in wordList are unique.
/// 

// # Solution
// Time complexity: O(N * M^2) 
// Space complexity: O(N * M) 

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set: HashSet<String> = word_list.into_iter().collect();
       
       if !word_set.contains(&end_word) {
           return 0;
       }
       
       let mut queue = VecDeque::new();
       queue.push_back((begin_word.clone(), 1));
       
       let mut visited = HashSet::new();
       visited.insert(begin_word);
       
       while let Some((current_word, level)) = queue.pop_front() {
           if current_word == end_word {
               return level;
           }
           
           let word_chars: Vec<char> = current_word.chars().collect();
           
           for i in 0..word_chars.len() {
               let mut new_chars = word_chars.clone();
               
               for c in b'a'..=b'z' {
                   new_chars[i] = c as char;
                   let new_word: String = new_chars.iter().collect();
                   
                   if word_set.contains(&new_word) && !visited.contains(&new_word) {
                       visited.insert(new_word.clone());
                       queue.push_back((new_word, level + 1));
                   }
               }
           }
       }
       
       0
    }
}