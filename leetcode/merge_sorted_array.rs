/// 
/// Problem: Merge Sorted Array
/// 
/// You are given two integer arrays `nums1` and `nums2`, sorted in **non-decreasing order**, and two integers `m` and `n`.
/// 
/// - `nums1` has a size of `m + n`, where the first `m` elements represent sorted numbers.
/// - The last `n` elements are set to `0` and should be ignored.
/// - Merge `nums2` into `nums1` in-place so that `nums1` remains sorted.
///
/// **Example 1:**
/// ```plaintext
/// Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
/// Output: [1,2,2,3,5,6]
/// Explanation: Merge `nums2` into `nums1` and maintain sorted order.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: nums1 = [1], m = 1, nums2 = [], n = 0
/// Output: [1]
/// ```
///
/// **Constraints:**
/// - `0 <= m, n <= 200`
/// - `nums1.length == m + n`
/// - `nums2.length == n`
/// - `-10^9 <= nums1[i], nums2[i] <= 10^9`
///
/// # Solution:
/// - **Time Complexity:** `O(m + n)`
/// - **Space Complexity:** `O(1)`
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j, mut k) = (m as usize, n as usize, (m + n) as usize);

        while j > 0 {
            if i > 0 && nums1[i - 1] > nums2[j - 1] {
                nums1[k - 1] = nums1[i - 1];
                i -= 1;
            } else {
                nums1[k - 1] = nums2[j - 1];
                j -= 1;
            }
            k -= 1;
        }
    }
}