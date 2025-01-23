/// 
/// Problem: Permutation Sequence
/// 
/// The set `{1, 2, 3, ..., n}` contains `n!` unique permutations.
/// Given `n` and `k`, return the `kᵗʰ` permutation sequence.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 3, k = 3
/// Output: "213"
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 4, k = 9
/// Output: "2314"
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: n = 3, k = 1
/// Output: "123"
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 9`
/// - `1 <= k <= n!`
///
/// # Solution:
/// - **Time Complexity:** `O(n²)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut numbers: Vec<u32> = (1..=n as u32).collect();
        let mut k = k as usize - 1;
        let mut fact = (1..=n as usize).product::<usize>();
        let mut result = String::new();

        for i in (1..=n as usize).rev() {
            fact /= i;
            let index = k / fact;
            result.push(char::from_digit(numbers[index], 10).unwrap());
            numbers.remove(index);
            k %= fact;
        }

        result
    }
}