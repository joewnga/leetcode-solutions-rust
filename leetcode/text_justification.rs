/// 
/// Problem: Text Justification
/// 
/// Given an array of words and a **maximum width `maxWidth`**, format the text so that each line:
/// - Has exactly `maxWidth` characters.
/// - Words are fully justified (evenly distributed).
/// - Extra spaces are placed between words **as evenly as possible**.
/// - If the number of spaces cannot be evenly distributed, the leftmost gaps get more spaces.
/// - The last line should be **left-justified** with no extra spaces between words.
///
/// **Example 1:**
/// ```plaintext
/// Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
/// Output:
/// [
///    "This    is    an",
///    "example  of text",
///    "justification.  "
/// ]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
/// Output:
/// [
///   "What   must   be",
///   "acknowledgment  ",
///   "shall be        "
/// ]
/// ```
///
/// **Constraints:**
/// - `1 <= words.length <= 300`
/// - `1 <= words[i].length <= 20`
/// - `1 <= maxWidth <= 100`
/// - Words consist of **only English letters**.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        
    let mut result = Vec::new();
    let max_width = max_width as usize;
    let mut index = 0;

    while index < words.len() {
        let mut line_length = words[index].len();
        let mut next_index = index + 1;

        
        while next_index < words.len() && line_length + words[next_index].len() + (next_index - index) <= max_width {
            line_length += words[next_index].len();
            next_index += 1;
        }

        let num_words = next_index - index;
        let num_gaps = num_words - 1;
        let total_spaces = max_width - line_length;

        let mut line = String::new();

        
        if next_index == words.len() || num_words == 1 {
            for i in index..next_index {
                line.push_str(&words[i]);
                if i < next_index - 1 {
                    line.push(' ');
                }
            }
            line.push_str(&" ".repeat(max_width - line.len()));
        } else {
            
            let spaces_per_gap = total_spaces / num_gaps;
            let extra_spaces = total_spaces % num_gaps;

            for i in index..next_index {
                line.push_str(&words[i]);
                if i < next_index - 1 {
                    let spaces = spaces_per_gap + if (i - index) < extra_spaces { 1 } else { 0 };
                    line.push_str(&" ".repeat(spaces));
                }
            }
        }

        result.push(line);
        index = next_index;
    }

    result
    }
}