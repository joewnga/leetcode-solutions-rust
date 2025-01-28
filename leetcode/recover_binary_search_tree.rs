/// 
/// Problem: Recover Binary Search Tree
/// 
/// You are given the `root` of a Binary Search Tree (BST), where **exactly two nodes are swapped** by mistake.
/// 
/// Recover the tree without changing its structure.
/// 
/// **Example 1:**
/// ```plaintext
/// Input: root = [1,3,null,null,2]
/// Output: [3,1,null,null,2]
/// Explanation: The nodes `1` and `3` were swapped.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: root = [3,1,4,null,null,2]
/// Output: [2,1,4,null,null,3]
/// Explanation: The nodes `2` and `3` were swapped.
/// ```
///
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[2, 1000]`.
/// - `-2^31 <= Node.val <= 2^31 - 1`
/// - The input tree is a **Binary Search Tree (BST)** where **two nodes have been swapped**.
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`
/// 
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut first, mut second, mut prev) = (None, None, None);
        Self::inorder(root, &mut first, &mut second, &mut prev);

        if let (Some(f), Some(s)) = (first, second) {
            std::mem::swap(&mut f.borrow_mut().val, &mut s.borrow_mut().val);
        }
    }

    fn inorder(
        node: &Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(n) = node {
            Self::inorder(&n.borrow().left, first, second, prev);

            
            if let Some(p) = prev {
                if p.borrow().val > n.borrow().val {
                    if first.is_none() {
                        *first = Some(Rc::clone(p)); 
                    }
                    *second = Some(Rc::clone(n)); 
                }
            }

            *prev = Some(Rc::clone(n)); 

            Self::inorder(&n.borrow().right, first, second, prev);
        }
    }
}