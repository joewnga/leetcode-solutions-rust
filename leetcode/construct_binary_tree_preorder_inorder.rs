/// 
/// **Problem:** Construct Binary Tree from Preorder and Inorder Traversal
///
/// Given two integer arrays `preorder` and `inorder`, construct and return the **binary tree**.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
/// Output: [3,9,20,null,null,15,7]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: preorder = [-1], inorder = [-1]
/// Output: [-1]
/// ```
///
/// **Constraints:**
/// - `1 <= preorder.length <= 3000`
/// - `inorder.length == preorder.length`
/// - `-3000 <= preorder[i], inorder[i] <= 3000`
/// - `preorder` and `inorder` consist of **unique values**.
/// - `inorder` represents a **valid binary tree**.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
//////////////////////////////////////////////////////////////////////////////////////////////////

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root_val = preorder[0];
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        let inorder_index = inorder.iter().position(|&x| x == root_val).unwrap();

        root.borrow_mut().left = Self::build_tree(
            preorder[1..=inorder_index].to_vec(), 
            inorder[..inorder_index].to_vec(),
        );

        root.borrow_mut().right = Self::build_tree(
            preorder[inorder_index + 1..].to_vec(), 
            inorder[inorder_index + 1..].to_vec(),
        );

        Some(root)
    }
}
