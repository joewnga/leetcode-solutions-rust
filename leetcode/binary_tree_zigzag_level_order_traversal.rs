/// 
/// Problem: Binary Tree Zigzag Level Order Traversal
/// 
/// Given the `root` of a binary tree, return its **zigzag level order traversal**.
/// 
/// That is, level order traversal **alternating** between left-to-right and right-to-left.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[3],[20,9],[15,7]]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: root = [1]
/// Output: [[1]]
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: root = []
/// Output: []
/// ```
///
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[0, 2000]`.
/// - `-1000 <= Node.val <= 1000`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::dfs(&root, 0, &mut result);
        result
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<Vec<i32>>) {
        if let Some(n) = node {
            let n = n.borrow();
            if level >= result.len() {
                result.push(Vec::new());
            }
            if level % 2 == 0 {
                result[level].push(n.val);
            } else {
                result[level].insert(0, n.val); 
            }
            Self::dfs(&n.left, level + 1, result);
            Self::dfs(&n.right, level + 1, result);
        }
    }
}