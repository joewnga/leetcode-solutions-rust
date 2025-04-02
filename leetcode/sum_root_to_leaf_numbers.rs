/// 
/// Problem: Sum Root to Leaf Numbers
/// 
/// You are given the root of a binary tree containing digits from 0 to 9 only.
/// Each root-to-leaf path in the tree represents a number.
/// 
/// For example, the root-to-leaf path 1->2->3 represents the number 123.
/// 
/// Return the total sum of all root-to-leaf numbers. Test cases are generated so that
/// the answer will fit in a 32-bit integer.
/// 
/// A leaf node is a node with no children.
/// 
/// Example 1:
/// Input: root = [1,2,3]
/// Output: 25
/// Explanation:
/// The root-to-leaf path 1->2 represents the number 12.
/// The root-to-leaf path 1->3 represents the number 13.
/// Therefore, sum = 12 + 13 = 25.
/// 
/// Example 2:
/// Input: root = [4,9,0,5,1]
/// Output: 1026
/// Explanation:
/// The root-to-leaf path 4->9->5 represents the number 495.
/// The root-to-leaf path 4->9->1 represents the number 491.
/// The root-to-leaf path 4->0 represents the number 40.
/// Therefore, sum = 495 + 491 + 40 = 1026.
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [1, 1000].
/// 0 <= Node.val <= 9
/// The depth of the tree will not exceed 10.
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
// Space complexity: O(h) 

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
           return 0;
       }
    
       let mut stack = Vec::new();
       stack.push((root.unwrap(), 0));
       
       let mut total_sum = 0;
       
       while let Some((node, current_sum)) = stack.pop() {
           let node_ref = node.borrow();
           
           let new_sum = current_sum * 10 + node_ref.val;
           
           if node_ref.left.is_none() && node_ref.right.is_none() {
               total_sum += new_sum;
           }
           
           if let Some(right) = &node_ref.right {
               stack.push((Rc::clone(right), new_sum));
           }
           
           if let Some(left) = &node_ref.left {
               stack.push((Rc::clone(left), new_sum));
           }
       }
       
       total_sum
    }
}