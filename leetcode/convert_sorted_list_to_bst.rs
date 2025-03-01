/// 
/// Problem: Convert Sorted List to Binary Search Tree
/// 
/// Given the head of a singly linked list where elements are sorted in ascending order, 
/// convert it to a height-balanced binary search tree.
/// 
/// A height-balanced binary tree is a binary tree in which the depth of the two subtrees 
/// of every node never differs by more than one.
/// 
/// Example 1:
/// Input: head = [-10,-3,0,5,9]
/// Output: [0,-3,9,-10,null,5]
/// Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown 
/// height balanced BST.
/// 
/// Example 2:
/// Input: head = []
/// Output: []
/// 
/// Constraints:
/// The number of nodes in head is in the range [0, 2 * 10^4].
/// -10^5 <= Node.val <= 10^5
/// 

// Definition for singly-linked list (provided by LeetCode)
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

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
// Time complexity: O(n) 
// Space complexity: O(n) 

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::new();
       let mut current = head;
       
       while let Some(node) = current {
           values.push(node.val);
           current = node.next;
       }
       
       
       Self::sorted_array_to_bst(&values, 0, values.len() as i32 - 1)
   }
   
   fn sorted_array_to_bst(nums: &[i32], left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
       if left > right {
           return None;
       }
       
       
       let mid = left + (right - left) / 2;
       let mut root = TreeNode::new(nums[mid as usize]);
       
       
       root.left = Self::sorted_array_to_bst(nums, left, mid - 1);
       root.right = Self::sorted_array_to_bst(nums, mid + 1, right);
       
       Some(Rc::new(RefCell::new(root)))
    }
}