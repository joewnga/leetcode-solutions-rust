/// 
/// Problem: Combination Sum
/// 
/// Given an array of **distinct** integers `candidates` and a `target` integer, find **all unique combinations** of `candidates` where the chosen numbers sum to `target`.
///
/// You may use **the same number multiple times** in a combination.
///
/// **Example 1:**
/// ```plaintext
/// Input: candidates = [2,3,6,7], target = 7
/// Output: [[2,2,3], [7]]
/// Explanation: 2+2+3=7 and 7=7.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: candidates = [2,3,5], target = 8
/// Output: [[2,2,2,2], [2,3,3], [3,5]]
/// ```
///
/// **Constraints:**
/// - `1 <= candidates.length <= 30`
/// - `1 <= candidates[i] <= 200`
/// - `All numbers are unique`
/// - `1 <= target <= 500`
///
/// # Solution:
///
/// - **Time Complexity:** `O(2^n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current_combination = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut current_combination, &mut result);
        result
    }

    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current_combination.clone());
            return;
        }

        for i in start..candidates.len() {
            if target < candidates[i] {
                continue;
            }

            current_combination.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], i, current_combination, result);
            current_combination.pop(); 
        }
    }
}