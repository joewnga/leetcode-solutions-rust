/// 
/// Problem: Gray Code
/// 
/// Given an integer `n`, return a **sequence** representing the **n-bit Gray code**.
/// 
/// A **Gray code sequence** must satisfy the following properties:
/// - The first number is `0`.
/// - Each **subsequent number differs from the previous one by exactly one bit**.
/// - The sequence should be of length `2^n`.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 2
/// Output: [0,1,3,2]
/// Explanation:
/// - `00 -> 00`
/// - `01 -> 01` (1-bit change)
/// - `11 -> 03` (1-bit change)
/// - `10 -> 02` (1-bit change)
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1
/// Output: [0,1]
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 16`
///
/// # Solution:
/// - **Time Complexity:** `O(2^n)`
/// - **Space Complexity:** `O(2^n)`
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|i| i ^ (i >> 1)).collect()
    }
}