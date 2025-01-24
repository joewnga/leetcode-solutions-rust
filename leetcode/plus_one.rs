/// 
/// Problem: Plus One
/// 
/// Given a **non-empty array of digits** representing a **non-negative integer**, **increment** the integer by one.
/// 
/// The **digits are stored such that the most significant digit is at the head of the list**, and each element contains a **single digit**.
/// 
/// You may assume **the integer does not contain any leading zero**, except for the number `0` itself.
///
/// **Example 1:**
/// ```plaintext
/// Input: digits = [1,2,3]
/// Output: [1,2,4]
/// Explanation: The array represents the integer `123`. After adding one, it becomes `124`.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: digits = [4,3,2,1]
/// Output: [4,3,2,2]
/// ```
///
/// **Example 3 (Edge Case: Carry Over):**
/// ```plaintext
/// Input: digits = [9,9,9]
/// Output: [1,0,0,0]
/// Explanation: The array represents `999`, and adding one results in `1000`.
/// ```
///
/// **Constraints:**
/// - `1 <= digits.length <= 100`
/// - `0 <= digits[i] <= 9`
///
/// # Solution:
/// - **Iterate from the last digit to the first, handling carry-over.**
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)` 
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        digits.insert(0, 1);
        digits
    }
}