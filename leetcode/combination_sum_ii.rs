/// 
/// Problem: Combination Sum II
/// 
/// Given a collection of candidate numbers (`candidates`) and a target number (`target`), find **all unique combinations** where the chosen numbers sum to `target`.
///
/// Each number in `candidates` **can only be used once** in the combination.
///
/// **Example 1:**
/// ```plaintext
/// Input: candidates = [10,1,2,7,6,1,5], target = 8
/// Output: [[1,1,6], [1,2,5], [1,7], [2,6]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: candidates = [2,5,2,1,2], target = 5
/// Output: [[1,2,2], [5]]
/// ```
///
/// **Constraints:**
/// - `1 <= candidates.length <= 100`
/// - `1 <= candidates[i] <= 50`
/// - `1 <= target <= 30`
/// - **Each number may only be used once in a combination.**
///
/// # Solution:
///
/// - **Time Complexity:** `O(2^n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
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
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }

            if target < candidates[i] {
                break; 
            }

            current_combination.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], i + 1, current_combination, result);
            current_combination.pop(); 
        }
    }
}