/// 
/// Problem: Word Ladder II
/// 
/// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a 
/// sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
/// 
/// - Every adjacent pair of words differs by a single letter.
/// - Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
/// - sk == endWord
/// 
/// Given two words, beginWord and endWord, and a dictionary wordList, return all the shortest 
/// transformation sequences from beginWord to endWord, or an empty list if no such sequence exists.
/// Each sequence should be returned as a list of the words [beginWord, s1, s2, ..., sk].
/// 
/// Example 1:
/// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
/// Output: [["hit","hot","dot","dog","cog"],["hit","hot","lot","log","cog"]]
/// Explanation: There are 2 shortest transformation sequences:
/// "hit" -> "hot" -> "dot" -> "dog" -> "cog"
/// "hit" -> "hot" -> "lot" -> "log" -> "cog"
/// 
/// Example 2:
/// Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
/// Output: []
/// Explanation: The endWord "cog" is not in wordList, so there is no valid transformation sequence.
/// 
/// Constraints:
/// 1 <= beginWord.length <= 5
/// endWord.length == beginWord.length
/// 1 <= wordList.length <= 500
/// wordList[i].length == beginWord.length
/// beginWord, endWord, and wordList[i] consist of lowercase English letters.
/// beginWord != endWord
/// All the words in wordList are unique.
/// 

// # Solution
// Time complexity: O(N * M^2) where N is the length of wordList and M is the length of each word
// Space complexity: O(N^2) 

use std::collections::{HashSet, HashMap, VecDeque};
impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
       
       
       if !word_set.contains(&end_word) {
           return vec![];
       }
       
       word_set.remove(&begin_word);
       
       let mut queue = VecDeque::new();
       queue.push_back(begin_word.clone());
       
       let mut parents: HashMap<String, Vec<String>> = HashMap::new();
       
       let mut current_level = HashSet::new();
       current_level.insert(begin_word.clone());
       
       let mut found = false;
       
       while !queue.is_empty() && !found {
           let level_size = queue.len();
           
           let mut level_visited = HashSet::new();
           
           for _ in 0..level_size {
               let current_word = queue.pop_front().unwrap();
               
               let word_chars: Vec<char> = current_word.chars().collect();
               
               for i in 0..word_chars.len() {
                   let mut new_chars = word_chars.clone();
                   
                   for c in b'a'..=b'z' {
                       new_chars[i] = c as char;
                       let new_word: String = new_chars.iter().collect();
                       
                       if !word_set.contains(&new_word) {
                           continue;
                       }
                       
                       parents.entry(new_word.clone())
                              .or_insert_with(Vec::new)
                              .push(current_word.clone());
                       
                       if !level_visited.contains(&new_word) {
                           level_visited.insert(new_word.clone());
                           
                           if new_word == end_word {
                               found = true;
                           } else {
                               queue.push_back(new_word);
                           }
                       }
                   }
               }
           }
           
           for word in &level_visited {
               word_set.remove(word);
           }
       }
    
       if !found {
           return vec![];
       }
       
       let mut result = Vec::new();
       let mut path = vec![end_word.clone()];
       
       Self::dfs(&end_word, &begin_word, &parents, &mut path, &mut result);
       
       result
   }
   
   fn dfs(word: &str, begin_word: &str, parents: &HashMap<String, Vec<String>>, 
          path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
       if word == begin_word {
           let mut complete_path = path.clone();
           complete_path.reverse();
           result.push(complete_path);
           return;
       }
       
       if let Some(parent_words) = parents.get(word) {
           for parent in parent_words {
               path.push(parent.clone());
               Self::dfs(parent, begin_word, parents, path, result);
               path.pop(); 
           }
       }
    }
}