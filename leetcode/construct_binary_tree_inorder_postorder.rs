/// 
/// Problem: Construct Binary Tree from Inorder and Postorder Traversal
/// 
/// Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree 
/// and postorder is the postorder traversal of the same tree, construct and return the binary tree.
/// 
/// Example 1:
/// Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
/// Output: [3,9,20,null,null,15,7]
/// 
/// Example 2:
/// Input: inorder = [-1], postorder = [-1]
/// Output: [-1]
/// 
/// Constraints:
/// 1 <= inorder.length <= 3000
/// postorder.length == inorder.length
/// -3000 <= inorder[i], postorder[i] <= 3000
/// inorder and postorder consist of unique values.
/// Each value of postorder also appears in inorder.
/// inorder is guaranteed to be the inorder traversal of the tree.
/// postorder is guaranteed to be the postorder traversal of the tree.
/// 

// Definition for a binary tree node (provided by LeetCode)
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

// # Solution: Recursive approach with linear search for root
// Time complexity: O(n²) 
// Space complexity: O(n) 

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_helper(&inorder, &postorder)
    }
    
    fn build_tree_helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        
        
        let root_val = postorder[postorder.len() - 1];
        let mut root = TreeNode::new(root_val);
        
        
        let root_pos = inorder.iter().position(|&x| x == root_val).unwrap();
        
        
        let right_inorder = &inorder[root_pos + 1..];
        let right_postorder = &postorder[postorder.len() - 1 - right_inorder.len()..postorder.len() - 1];
        root.right = Self::build_tree_helper(right_inorder, right_postorder);
        
        
        let left_inorder = &inorder[0..root_pos];
        let left_postorder = &postorder[0..left_inorder.len()];
        root.left = Self::build_tree_helper(left_inorder, left_postorder);
        
        Some(Rc::new(RefCell::new(root)))
    }
}
