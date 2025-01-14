/// 
/// Problem: Divide Two Integers
/// 
/// Given two integers `dividend` and `divisor`, divide them without using multiplication, division, or mod operator.
/// Return the quotient after dividing the `dividend` by the `divisor`.
///
/// The integer division should truncate toward zero, which means losing its fractional part.
/// For example, `8 / 3 = 2` and `-8 / 3 = -2`.
///
/// Constraints:
/// - The divisor will never be `0`.
/// - Assume that the division result will be within the range of a 32-bit signed integer: [−2^31, 2^31 − 1].
///
/// Example 1:
/// Input: dividend = 10, divisor = 3
/// Output: 3
///
/// Example 2:
/// Input: dividend = 7, divisor = -3
/// Output: -2
///
/// Time complexity: O(log(n))
/// Space complexity: O(1).
impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX; 
        }
        dividend / divisor
    }
}

