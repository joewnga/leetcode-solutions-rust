/// 
/// Problem: Same Tree
/// 
/// Given the roots of two binary trees `p` and `q`, determine if they are **structurally identical** and have **the same node values**.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: p = [1,2,3], q = [1,2,3]
/// Output: true
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: p = [1,2], q = [1,null,2]
/// Output: false
/// ```
///
/// **Constraints:**
/// - The number of nodes in both trees is in the range `[0, 100]`.
/// - `-10^4 <= Node.val <= 10^4`
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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true, 
            (Some(n1), Some(n2)) => {
                let (n1, n2) = (n1.borrow(), n2.borrow());
                n1.val == n2.val
                    && Self::is_same_tree(n1.left.clone(), n2.left.clone())
                    && Self::is_same_tree(n1.right.clone(), n2.right.clone())
            }
            _ => false, 
        }
    }
}