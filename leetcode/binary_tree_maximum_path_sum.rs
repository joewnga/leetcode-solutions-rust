/// 
/// Problem: Binary Tree Maximum Path Sum
/// 
/// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the 
/// sequence has an edge connecting them. A node can only appear in the sequence at most once. 
/// Note that the path does not need to pass through the root.
/// 
/// The path sum of a path is the sum of the node's values in the path.
/// 
/// Given the root of a binary tree, return the maximum path sum of any non-empty path.
/// 
/// Example 1:
/// Input: root = [1,2,3]
/// Output: 6
/// Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
/// 
/// Example 2:
/// Input: root = [-10,9,20,null,null,15,7]
/// Output: 42
/// Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [1, 3 * 10^4].
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
// Space complexity: O(h) - Where h is the height of the tree

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = std::i32::MIN;
       Self::max_gain(root, &mut max_sum);
       max_sum
   }
   
   
   fn max_gain(node: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
       if let Some(n) = node {
           let node_ref = n.borrow();
           
           let left_gain = cmp::max(0, Self::max_gain(node_ref.left.clone(), max_sum));
           let right_gain = cmp::max(0, Self::max_gain(node_ref.right.clone(), max_sum));
           
           let price_new_path = node_ref.val + left_gain + right_gain;
           
           *max_sum = cmp::max(*max_sum, price_new_path);
           
           node_ref.val + cmp::max(left_gain, right_gain)
       } else {
           0
       }
    }
}