///
///  Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
/// Follow up: Could you solve it without converting the integer to a string?
/// 
/// Example 1:
/// Input: x = 121
/// Output: true
/// 


/// ### Complexity
/// 
/// - Time: O(log10(n))
/// - Space: O(1)


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {

        if x < 0  || (x % 10 == 0 && x != 0){ return false; }
        let mut original = x;
        let mut reversed_half = 0;

        while original > reversed_half {

            
            reversed_half = reversed_half * 10 + original % 10;
            // 1st iter: 0 * 10 + 121 % 10 = 0 + 1 = 1
            // 2nd iter: 1 * 10 + 12 % 10 = 10 + 2 = 12
            original /= 10;
            // 1st iter: 121 / 10 = 12
            // 2nd iter: 12 / 10 = 1

            
        }

        original == reversed_half || original == reversed_half / 10

        
    }
}