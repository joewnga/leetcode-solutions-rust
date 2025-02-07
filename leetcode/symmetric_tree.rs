/// 
/// Problem: Symmetric Tree
/// 
/// Given the `root` of a binary tree, determine if it is **symmetric** around its center.
/// 
/// A tree is symmetric if the left subtree is a **mirror reflection** of the right subtree.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: root = [1,2,2,3,4,4,3]
/// Output: true
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: root = [1,2,2,null,3,null,3]
/// Output: false
/// ```
///
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[0, 1000]`.
/// - `-100 <= Node.val <= 100`
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        Self::is_mirror(
            root.as_ref().unwrap().borrow().left.clone(),
            root.as_ref().unwrap().borrow().right.clone(),
        )
    }

    fn is_mirror(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (None, None) => true, 
            (Some(n1), Some(n2)) => {
                let (n1, n2) = (n1.borrow(), n2.borrow());
                n1.val == n2.val 
                    && Self::is_mirror(n1.left.clone(), n2.right.clone()) 
                    && Self::is_mirror(n1.right.clone(), n2.left.clone()) 
            }
            _ => false,
        }
    }
}