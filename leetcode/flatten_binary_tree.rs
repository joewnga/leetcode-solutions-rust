/// 
/// Problem: Flatten Binary Tree to Linked List
/// 
/// Given the root of a binary tree, flatten the tree into a "linked list":
/// 
/// The "linked list" should use the same TreeNode class where the right child pointer points
/// to the next node in the list and the left child pointer is always null.
/// 
/// The "linked list" should be in the same order as a pre-order traversal of the binary tree.
/// 
/// Example 1:
/// Input: root = [1,2,5,3,4,null,6]
/// Output: [1,null,2,null,3,null,4,null,5,null,6]
/// 
/// Example 2:
/// Input: root = []
/// Output: []
/// 
/// Example 3:
/// Input: root = [0]
/// Output: [0]
/// 
/// Constraints:
/// The number of nodes in the tree is in the range [0, 2000].
/// -100 <= Node.val <= 100
/// 
/// Follow up: Can you flatten the tree in-place (with O(1) extra space)?
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
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut nodes = Vec::new();
       Self::preorder_traversal(root.clone(), &mut nodes);
       
       // Flatten the tree
       if let Some(current_node) = root {
           let mut current_ref = current_node.borrow_mut();
           
           // Start with the first node
           for i in 1..nodes.len() {
               // Set right child to the next node in preorder
               current_ref.right = Some(Rc::clone(&nodes[i]));
               current_ref.left = None;
               
               // Move to the next node
               current_ref = nodes[i].borrow_mut();
           }
           
           // Set the leaf node's pointers
           current_ref.left = None;
           current_ref.right = None;
       }
   }
   
   fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>) {
       if let Some(node) = root {
           nodes.push(Rc::clone(&node));
           
           let node_ref = node.borrow();
           Self::preorder_traversal(node_ref.left.clone(), nodes);
           Self::preorder_traversal(node_ref.right.clone(), nodes);
       }
    }
}