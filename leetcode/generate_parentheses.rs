///
/// Problem: Generate Parentheses
///
/// Given `n` pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
///
/// Example 1:
/// Input: n = 3
/// Output: ["((()))","(()())","(())()","()(())","()()()"]
///
/// Example 2:
/// Input: n = 1
/// Output: ["()"]
///
/// # Solution:
///
/// Time complexity: O(4^n)
/// Space complexity: O(n)


impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        fn backtrack(
            current: &mut String,
            open: i32,
            close: i32,
            max: i32,
            result: &mut Vec<String>,
        ) {
            
            if current.len() as i32 == max * 2 {
                result.push(current.clone());
                return;
            }

            
            if open < max {
                current.push('(');
                backtrack(current, open + 1, close, max, result);
                current.pop(); 
            }

            if close < open {
                current.push(')');
                backtrack(current, open, close + 1, max, result);
                current.pop();
            }
        }

        backtrack(&mut String::new(), 0, 0, n, &mut result);

        result
    }
}
