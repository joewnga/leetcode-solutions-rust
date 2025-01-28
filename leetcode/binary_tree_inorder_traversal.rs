/// 
/// Problem: Binary Tree Inorder Traversal
/// 
/// Given the root of a binary tree, return the inorder traversal of its nodes' values.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: root = [1,null,2,3]
/// Output: [1,3,2]
/// Explanation: Inorder traversal visits nodes in the order: left → root → right.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: root = []
/// Output: []
/// ```
///
/// **Example 3:**
/// ```plaintext
/// Input: root = [1]
/// Output: [1]
/// ```
///
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[0, 100]`.
/// - `-100 <= Node.val <= 100`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
/// 
/// 
// Definition for a binary tree node.
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::dfs(&root, &mut result);
        result
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::dfs(&n.left, result);  
            result.push(n.val);          
            Self::dfs(&n.right, result); 
        }
    }
}