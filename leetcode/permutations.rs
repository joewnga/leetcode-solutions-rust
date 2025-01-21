/// 
/// Problem: Permutations
/// 
/// Given an array `nums` of **distinct** integers, return **all possible permutations** in any order.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [1,2,3]
/// Output: [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [0,1]
/// Output: [[0,1], [1,0]]
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: nums = [1]
/// Output: [[1]]
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 6`
/// - `-10 <= nums[i] <= 10`
/// - **All numbers are distinct.**
///
/// # Solution:
/// - **Backtracking approach** is optimal.
/// - **Swap elements in-place** to generate permutations efficiently.
/// - **Time Complexity:** `O(n!)`, since there are `n!` permutations.
/// - **Space Complexity:** `O(n)`, for recursion depth.
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut nums = nums;
        Self::backtrack(0, &mut nums, &mut result);
        result
    }

    fn backtrack(start: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i);
            Self::backtrack(start + 1, nums, result);
            nums.swap(start, i); 
        }
    }
}