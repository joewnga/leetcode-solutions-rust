/// 
/// Problem: Edit Distance
/// 
/// Given two strings `word1` and `word2`, return the **minimum number of operations required** to convert `word1` to `word2`.
///
/// **Operations Allowed:**
/// 1. **Insert a character**.
/// 2. **Delete a character**.
/// 3. **Replace a character**.
///
/// **Example 1:**
/// ```plaintext
/// Input: word1 = "horse", word2 = "ros"
/// Output: 3
/// Explanation:
/// horse → rorse (replace 'h' → 'r')
/// rorse → rose  (delete 'r')
/// rose → ros    (delete 'e')
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: word1 = "intention", word2 = "execution"
/// Output: 5
/// Explanation:
/// intention → exention (replace 'i' → 'e')
/// exention → exection  (replace 'n' → 'c')
/// exection → execution (insert 'u')
/// execution → execution (no change)
/// ```
///
/// **Constraints:**
/// - `0 <= word1.length, word2.length <= 500`
/// - `word1` and `word2` consist of **lowercase English letters**.
///
/// # Solution:
/// - **Time Complexity:** `O(m * n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();

        let mut prev = vec![0; n + 1];

        for j in 0..=n {
            prev[j] = j as i32;
        }

        for i in 1..=m {
            let mut curr = vec![0; n + 1];
            curr[0] = i as i32;

            for j in 1..=n {
                if word1[i - 1] == word2[j - 1] {
                    curr[j] = prev[j - 1];
                } else {
                    curr[j] = 1 + prev[j - 1].min(prev[j]).min(curr[j - 1]);
                }
            }

            prev = curr;
        }

        prev[n]
    }
}