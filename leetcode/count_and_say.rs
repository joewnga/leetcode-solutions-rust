/// 
/// Problem: Count and Say
/// 
/// The **Count and Say** sequence is a series of strings where:
/// - `count_and_say(1) = "1"`
/// - `count_and_say(n)` is derived from `count_and_say(n-1)` by **reading** it aloud.
///
/// **Example Sequence:**
/// ```plaintext
/// count_and_say(1) = "1"
/// count_and_say(2) = "11"  -> One "1"
/// count_and_say(3) = "21"  -> Two "1"s
/// count_and_say(4) = "1211" -> One "2", One "1"
/// count_and_say(5) = "111221" -> One "1", One "1", Two "2", One "1"
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 30`
///
///
/// **Time Complexity:** `O(2^n)`
/// **Space Complexity:** `O(2^n)`

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = String::from("1");

        for _ in 1..n {
            let mut next_result = String::new();
            let mut chars = result.chars().peekable();
            while let Some(ch) = chars.next() {
                let mut count = 1;
                while chars.peek() == Some(&ch) {
                    chars.next();
                    count += 1;
                }
                next_result.push_str(&format!("{}{}", count, ch));
            }
            result = next_result;
        }

        result
    }
}