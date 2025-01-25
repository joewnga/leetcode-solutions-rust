/// 
/// Problem: Simplify Path
/// 
/// Given a **string path**, which is an absolute Unix-style path, return its **simplified canonical path**.
///
/// **Rules:**
/// - Multiple slashes (`/`) are treated as a **single slash**.
/// - `"."` refers to the **current directory** and can be ignored.
/// - `".."` refers to the **parent directory** (move up one level).
/// - The resulting canonical path **must always start with `/`**.
/// - The canonical path **must not contain trailing slashes** unless it's the root.
///
/// **Example 1:**
/// ```plaintext
/// Input: path = "/home/"
/// Output: "/home"
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: path = "/../"
/// Output: "/"
/// Explanation: ".." moves up a directory, but at the root, it stays "/".
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: path = "/home//foo/"
/// Output: "/home/foo"
/// ```
///
/// **Example 4:**
/// ```plaintext
/// Input: path = "/a/./b/../../c/"
/// Output: "/c"
/// ```
///
/// **Constraints:**
/// - `1 <= path.length <= 3000`
/// - `path` consists only of English letters, digits, `'.'`, `'/'`, and `'_'`.
/// - `path` always **starts with `'/'`**.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        
        for dir in path.split('/') {
            match dir {
                "" | "." => continue,  
                ".." => { stack.pop(); }
                _ => stack.push(dir), 
            }
        }
        
        format!("/{}", stack.join("/"))
    }
}