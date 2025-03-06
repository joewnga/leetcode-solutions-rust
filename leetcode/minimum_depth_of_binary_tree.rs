/// 
/// Problem: Minimum Depth of Binary Tree
/// 
/// Given a binary tree, find its minimum depth.
/// 
/// The minimum depth is the number of nodes along the shortest path from the root node 
/// down to the nearest leaf node.
/// 
/// Note: A leaf is a node with no children.
/// 
/// Example 1:
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 2
/// 
/// Example 2:
/// Input: root = [2,null,3,null,4,null,5,null,6]
/// Output: 5
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [0, 10^5].
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

// # Solution
// Time complexity: O(n) 
// Space complexity: O(h) Where h is the height of the tree (for the recursion stack)


use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
         match root {
           None => 0,
           Some(node) => {
               let node_ref = node.borrow();
               let left_depth = Self::min_depth(node_ref.left.clone());
               let right_depth = Self::min_depth(node_ref.right.clone());
               
               
               if left_depth == 0 {
                   return right_depth + 1;
               }
               if right_depth == 0 {
                   return left_depth + 1;
               }
               
               
               1 + cmp::min(left_depth, right_depth)
           }
       }
    }
}