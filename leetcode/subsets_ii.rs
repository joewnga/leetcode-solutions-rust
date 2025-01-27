/// 
/// Problem: Subsets II
/// 
/// Given an integer array `nums` that **may contain duplicates**, return all possible **unique subsets**.
/// 
/// The solution must not contain duplicate subsets.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums = [1,2,2]
/// Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums = [0]
/// Output: [[],[0]]
/// ```
///
/// **Constraints:**
/// - `1 <= nums.length <= 10`
/// - `-10 <= nums[i] <= 10`
///
/// # Solution:
/// - **Time Complexity:** `O(2^n)`
/// - **Space Complexity:** `O(2^n)`
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); 
        let mut result = Vec::new();
        let mut subset = Vec::new();
        Self::backtrack(0, &nums, &mut subset, &mut result);
        result
    }

    fn backtrack(start: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(subset.clone()); 

        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue; 
            }
            subset.push(nums[i]); 
            Self::backtrack(i + 1, nums, subset, result); 
            subset.pop(); 
        }
    }
}