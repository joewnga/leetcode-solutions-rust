/// 
/// Problem: Letter Combinations of a Phone Number
/// 
/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
/// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
/// 
/// Example 1:
/// Input: digits = "23"
/// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
/// 
/// Example 2:
/// Input: digits = ""
/// Output: []
/// Explanation: No digits to process, so return an empty array.
///
/// Example 3:
/// Input: digits = "2"
/// Output: ["a","b","c"]
/// 
/// # Solution:
/// 
/// Time complexity: O(4^n * n)
/// Space complexity: O(n)

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![]; 
        }

        let phone_map = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
        ];

        let mut result = Vec::new(); 
        let mut current_combination = String::new(); 

        fn backtrack(
            index: usize,
            digits: &Vec<char>,
            phone_map: &Vec<&str>,
            current_combination: &mut String,
            result: &mut Vec<String>,
        ) {
            
            if index == digits.len() {
                result.push(current_combination.clone()); 
                return;
            }

            
            let digit = digits[index].to_digit(10).unwrap() as usize;
            let letters = phone_map[digit];

            for letter in letters.chars() {
                current_combination.push(letter); 
                backtrack(index + 1, digits, phone_map, current_combination, result); 
                current_combination.pop(); 
            }
        }

        let digits: Vec<char> = digits.chars().collect();
        backtrack(0, &digits, &phone_map, &mut current_combination, &mut result);

        result
    }
}
