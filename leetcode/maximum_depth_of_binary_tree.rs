/// 
/// Problem: Maximum Depth of Binary Tree
/// 
/// Given the `root` of a binary tree, return its **maximum depth**.
/// 
/// The maximum depth is the number of nodes along the **longest path** from the root down to the farthest leaf node.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 3
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: root = [1,null,2]
/// Output: 2
/// ```
///
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[0, 10^4]`.
/// - `-100 <= Node.val <= 100`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
///////////////////////////////////////////////////////////////////////////////


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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0, 
            Some(node) => {
                let node = node.borrow();
                1 + i32::max(Self::max_depth(node.left.clone()), Self::max_depth(node.right.clone()))
            }
        }
    }
}