/// 
/// Problem: Add Binary
/// 
/// Given two binary strings `a` and `b`, return their sum as a binary string.
///
/// **Example 1:**
/// ```plaintext
/// Input: a = "11", b = "1"
/// Output: "100"
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: a = "1010", b = "1011"
/// Output: "10101"
/// ```
///
/// **Constraints:**
/// - `1 <= a.length, b.length <= 10^4`
/// - `a` and `b` consist only of `'0'` or `'1'`.
/// - `a` and `b` **do not contain leading zeros**, except for `"0"`.
///
/// # Solution:
/// - **Iterate from right to left and sum bits manually.**
/// - **Time Complexity:** `O(max(m, n))`
/// - **Space Complexity:** `O(max(m, n))`
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;
        let mut result = String::new();

        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut i = a.len() as isize - 1;
        let mut j = b.len() as isize - 1;

        while i >= 0 || j >= 0 || carry > 0 {
            let bit_a = if i >= 0 { (a_bytes[i as usize] - b'0') as i32 } else { 0 };
            let bit_b = if j >= 0 { (b_bytes[j as usize] - b'0') as i32 } else { 0 };
            
            let sum = bit_a + bit_b + carry;
            result.push((b'0' + (sum % 2) as u8) as char);
            carry = sum / 2;
            
            i -= 1;
            j -= 1;
        }

        result.chars().rev().collect()
    }
}