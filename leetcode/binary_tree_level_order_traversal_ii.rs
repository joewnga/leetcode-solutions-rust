/// 
/// Problem: Binary Tree Level Order Traversal II
/// 
/// Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values.
/// (i.e., from left to right, level by level from leaf to root).
/// 
/// Example 1:
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[15,7],[9,20],[3]]
/// 
/// Example 2:
/// Input: root = [1]
/// Output: [[1]]
/// 
/// Example 3:
/// Input: root = []
/// Output: []
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [0, 2000].
/// -1000 <= Node.val <= 1000
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

// # Solution : 
// Time complexity: O(n) 
// Space complexity: O(h) - Where h is the height of the tree (for the recursion stack)
//                    

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
   pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
       let mut result: Vec<Vec<i32>> = Vec::new();
       Self::dfs(root, 0, &mut result);
       result.reverse();
       result
   }
   
   fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<Vec<i32>>) {
       if let Some(n) = node {
           // Ensure we have a vector for this level
           if result.len() <= level {
               result.push(Vec::new());
           }
           
           // Add the current node's value to the appropriate level
           let n_ref = n.borrow();
           result[level].push(n_ref.val);
           
           // Recursively process left and right subtrees
           Self::dfs(n_ref.left.clone(), level + 1, result);
           Self::dfs(n_ref.right.clone(), level + 1, result);
       }
   }
}