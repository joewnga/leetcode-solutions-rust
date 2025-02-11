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
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let index_map: HashMap<_, _> = inorder.iter().enumerate().map(|(i, &val)| (val, i as i32)).collect();
        let mut pre_idx = 0;
        Self::build_tree_helper(&preorder, &mut pre_idx, 0, inorder.len() as i32 - 1, &index_map)
    }

    fn build_tree_helper(
        preorder: &[i32],
        pre_idx: &mut usize,
        in_start: i32,
        in_end: i32,
        index_map: &HashMap<i32, i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_start > in_end {
            return None;
        }

        let root_val = preorder[*pre_idx];
        *pre_idx += 1;

        let mut root = TreeNode::new(root_val);
        let in_root = index_map[&root_val];

        root.left = Self::build_tree_helper(preorder, pre_idx, in_start, in_root - 1, index_map);
        root.right = Self::build_tree_helper(preorder, pre_idx, in_root + 1, in_end, index_map);

        Some(Rc::new(RefCell::new(root)))
    }
}
