/// 
/// # Problem
/// 
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
/// 
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
/// For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
/// 
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
/// 
/// I can be placed before V (5) and X (10) to make 4 and 9.
/// X can be placed before L (50) and C (100) to make 40 and 90.
/// C can be placed before D (500) and M (1000) to make 400 and 900.
/// Given an integer, convert it to a roman numeral.
///
/// Example 1:
/// 
/// Input: num = 3
/// Output: "III"
/// 

/// # Solution 1:
/// Time complexity: O(1)
/// Space complexity: O(1)

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut result = String::new();
        let roman_numerals = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for (value, symbol) in roman_numerals {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }
        result
    }
}