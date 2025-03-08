/// 
/// Problem: Path Sum II
/// 
/// Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths 
/// where the sum of the node values in the path equals targetSum. Each path should be 
/// returned as a list of the node values, not node references.
/// 
/// A root-to-leaf path is a path starting from the root and ending at any leaf node. 
/// A leaf is a node with no children.
/// 
/// Example 1:
/// Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
/// Output: [[5,4,11,2],[5,8,4,5]]
/// Explanation: There are two paths whose sum equals targetSum:
/// 5 + 4 + 11 + 2 = 22
/// 5 + 8 + 4 + 5 = 22
/// 
/// Example 2:
/// Input: root = [1,2,3], targetSum = 5
/// Output: []
/// 
/// Example 3:
/// Input: root = [1,2], targetSum = 0
/// Output: []
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [0, 5000].
/// -1000 <= Node.val <= 1000
/// -1000 <= targetSum <= 1000
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
// Time complexity: O(n²) 
// Space complexity: O(h) - Where h is the height of the tree 

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
       let mut current_path = Vec::new();
       
       Self::dfs(root, target_sum, &mut current_path, &mut result);
       
       result
   }
   
   fn dfs(
       node: Option<Rc<RefCell<TreeNode>>>, 
       remaining: i32, 
       current_path: &mut Vec<i32>, 
       result: &mut Vec<Vec<i32>>
   ) {
       if let Some(n) = node {
           let n_ref = n.borrow();
           let val = n_ref.val;
           
           
           current_path.push(val);
           
           
           if n_ref.left.is_none() && n_ref.right.is_none() && val == remaining {
               result.push(current_path.clone());
           }
           
           
           Self::dfs(n_ref.left.clone(), remaining - val, current_path, result);
           Self::dfs(n_ref.right.clone(), remaining - val, current_path, result);
           
           
           current_path.pop();
       }
    }
}

