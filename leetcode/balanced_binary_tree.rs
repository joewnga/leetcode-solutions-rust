/// 
/// Problem: Balanced Binary Tree
/// 
/// Given a binary tree, determine if it is height-balanced.
/// 
/// A height-balanced binary tree is defined as a binary tree in which the depth of the 
/// two subtrees of every node never differ by more than 1.
/// 
/// Example 1:
/// Input: root = [3,9,20,null,null,15,7]
/// Output: true
/// 
/// Example 2:
/// Input: root = [1,2,2,3,3,null,null,4,4]
/// Output: false
/// 
/// Example 3:
/// Input: root = []
/// Output: true
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [0, 5000].
/// -10^4 <= Node.val <= 10^4
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

// # Solution:
// Time complexity: O(n²)
// Space complexity: O(n)

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
           return true;
       }
       
       let node = root.unwrap();
       let node_ref = node.borrow();
       
       // Get heights of left and right subtrees
       let left_height = Self::height(node_ref.left.clone());
       let right_height = Self::height(node_ref.right.clone());
       
       // Check if current node is balanced and recursively check children
       (left_height - right_height).abs() <= 1 
           && Self::is_balanced(node_ref.left.clone()) 
           && Self::is_balanced(node_ref.right.clone())
   }
   
   
   fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
       if root.is_none() {
           return 0;
       }
       
       let node = root.unwrap();
       let node_ref = node.borrow();
       
       1 + cmp::max(
           Self::height(node_ref.left.clone()),
           Self::height(node_ref.right.clone())
       )
    }
}