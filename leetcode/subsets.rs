/// 
/// Problem: Subsets
/// 
/// Given an integer array `nums`, return **all possible subsets (the power set)**.
/// 
/// The solution set must not contain duplicate subsets.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [1,2,3]
/// Output:
/// [
///   [], [1], [2], [3],
///   [1,2], [1,3], [2,3],
///   [1,2,3]
/// ]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [0]
/// Output: [[], [0]]
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10`
/// - `-10 <= nums[i] <= 10`
/// - `All elements of nums are unique.`
///
/// # Solution:
/// - **Time Complexity:** `O(2^n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut subset = Vec::new();
        Self::backtrack(0, &nums, &mut subset, &mut result);
        result
    }

    fn backtrack(start: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(subset.clone());

        for i in start..nums.len() {
            subset.push(nums[i]);
            Self::backtrack(i + 1, nums, subset, result);
            subset.pop();
        }
    }
}