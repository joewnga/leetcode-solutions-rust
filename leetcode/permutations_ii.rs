/// 
/// Problem: Permutations II
/// 
/// Given a collection of numbers, `nums`, that might contain **duplicates**, return **all unique permutations** in any order.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [1,1,2]
/// Output: [[1,1,2], [1,2,1], [2,1,1]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [1,2,3]
/// Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 8`
/// - `-10 <= nums[i] <= 10`
///
/// # Solution:
///
/// - **Time Complexity:** `O(n!)`
/// - **Space Complexity:** `O(n)`
use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); 
        let mut result = Vec::new();
        let mut used = vec![false; nums.len()];
        let mut current_permutation = Vec::new();
        Self::backtrack(&nums, &mut used, &mut current_permutation, &mut result);
        result
    }

    fn backtrack(
        nums: &Vec<i32>,
        used: &mut Vec<bool>,
        current_permutation: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if current_permutation.len() == nums.len() {
            result.push(current_permutation.clone());
            return;
        }

        let mut seen = HashSet::new();
        for i in 0..nums.len() {
            if used[i] || seen.contains(&nums[i]) {
                continue;
            }

            used[i] = true;
            seen.insert(nums[i]);
            current_permutation.push(nums[i]);
            Self::backtrack(nums, used, current_permutation, result);
            current_permutation.pop();
            used[i] = false;
        }
    }
}