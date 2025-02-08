/// 
/// Problem: Binary Tree Level Order Traversal
/// 
/// Given the `root` of a binary tree, return the **level order traversal** of its nodes' values.
/// 
/// (i.e., from left to right, level by level).
/// 
/// **Example 1:**
/// ```plaintext
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[3],[9,20],[15,7]]
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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut level = Vec::new();

            for _ in 0..level_size {
                if let Some(Some(node)) = queue.pop_front() {
                    let node = node.borrow();
                    level.push(node.val);
                    if node.left.is_some() {
                        queue.push_back(node.left.clone());
                    }
                    if node.right.is_some() {
                        queue.push_back(node.right.clone());
                    }
                }
            }

            result.push(level);
        }

        result
    }
}