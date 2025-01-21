/// 
/// Problem: Multiply Strings
/// 
/// Given two non-negative integers `num1` and `num2` represented as strings, return the product as a string.
///
/// **You must not use any built-in multiplication function.**
///
/// **Example 1:**
/// ```plaintext
/// Input: num1 = "2", num2 = "3"
/// Output: "6"
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: num1 = "123", num2 = "456"
/// Output: "56088"
/// ```
///
/// **Constraints:**
/// - `1 <= num1.length, num2.length <= 200`
/// - `num1` and `num2` **contain only digits ('0'-'9')**.
/// - `num1` and `num2` **do not contain leading zeros**, except `"0"` itself.
///
/// # Solution:
///
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(m + n)`
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let num1: Vec<u8> = num1.bytes().map(|b| b - b'0').collect();
        let num2: Vec<u8> = num2.bytes().map(|b| b - b'0').collect();
        let mut result = vec![0; num1.len() + num2.len()];

        for i in (0..num1.len()).rev() {
            for j in (0..num2.len()).rev() {
                let mul = (num1[i] as usize) * (num2[j] as usize);
                let sum = mul + result[i + j + 1];

                result[i + j + 1] = sum % 10;
                result[i + j] += sum / 10;
            }
        }

        let mut result_str = String::new();
        let mut leading_zero = true;

        for &digit in &result {
            if digit == 0 && leading_zero {
                continue;
            }
            leading_zero = false;
            result_str.push((digit as u8 + b'0') as char);
        }

        result_str
        
    }
}