/// 
/// Problem: Validate Binary Search Tree
/// 
/// Given the `root` of a binary tree, determine if it is a **valid** Binary Search Tree (BST).
/// 
/// **A valid BST must satisfy the following conditions:**
/// 1. The left subtree of a node contains **only nodes with values less than the node’s value**.
/// 2. The right subtree of a node contains **only nodes with values greater than the node’s value**.
/// 3. **Both left and right subtrees must also be valid BSTs.**
///
/// **Example 1:**
/// ```plaintext
/// Input: root = [2,1,3]
/// Output: true
/// Explanation: The tree is a valid BST.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: root = [5,1,4,null,null,3,6]
/// Output: false
/// Explanation: The left child of node 4 is 3, which is invalid.
/// ```
///
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[1, 10^4]`.
/// - `-2^31 <= Node.val <= 2^31 - 1`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev = None;
        Self::inorder(&root, &mut prev)
    }

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
        if let Some(n) = node {
            let n = n.borrow();
            if !Self::inorder(&n.left, prev) {
                return false;
            }
            if let Some(p) = prev {
                if n.val <= *p {
                    return false;
                }
            }
            *prev = Some(n.val);
            Self::inorder(&n.right, prev)
        } else {
            true
        }
    }
}