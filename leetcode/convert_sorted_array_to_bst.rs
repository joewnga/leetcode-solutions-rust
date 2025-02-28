/// 
/// Problem: Convert Sorted Array to Binary Search Tree
/// 
/// Given an integer array nums where the elements are sorted in ascending order, 
/// convert it to a height-balanced binary search tree.
/// 
/// A height-balanced binary tree is a binary tree in which the depth of the two 
/// subtrees of every node never differs by more than one.
/// 
/// Example 1:
/// Input: nums = [-10,-3,0,5,9]
/// Output: [0,-3,9,-10,null,5]
/// Explanation: [0,-10,5,null,-3,null,9] is also accepted.
/// 
/// Example 2:
/// Input: nums = [1,3]
/// Output: [3,1]
/// 
/// Constraints:
/// 1 <= nums.length <= 10^4
/// -10^4 <= nums[i] <= 10^4
/// nums is sorted in a strictly increasing order.
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
// Space complexity: O(log n) 


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums, 0, nums.len() as i32 - 1)
   }
   
   fn helper(nums: &[i32], left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
       if left > right {
           return None;
       }
       
       
       let mid = left + (right - left) / 2;
       
       
       let mut root = TreeNode::new(nums[mid as usize]);
       
       
       root.left = Self::helper(nums, left, mid - 1);
       root.right = Self::helper(nums, mid + 1, right);
       
       Some(Rc::new(RefCell::new(root)))
    }
}