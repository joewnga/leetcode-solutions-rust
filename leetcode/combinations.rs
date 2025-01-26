/// 
/// Problem: Combinations
/// 
/// Given two integers `n` and `k`, return **all possible combinations** of `k` numbers chosen from the range `[1, n]`.
/// 
/// You may return the answer **in any order**.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 4, k = 2
/// Output:
/// [
///   [1,2], [1,3], [1,4],
///   [2,3], [2,4],
///   [3,4]
/// ]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1, k = 1
/// Output: [[1]]
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 20`
/// - `1 <= k <= n`
///
/// # Solution:
/// - **Time Complexity:** `O(n^k)`
/// - **Space Complexity:** `O(k)`
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        Self::backtrack(1, n, k, &mut combination, &mut result);
        result
    }

    fn backtrack(start: i32, n: i32, k: i32, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if combination.len() == k as usize {
            result.push(combination.clone());
            return;
        }

        for i in start..=n {
            combination.push(i);
            Self::backtrack(i + 1, n, k, combination, result);
            combination.pop();
        }
    }
}