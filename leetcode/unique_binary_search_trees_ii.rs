/// 
/// Problem: Unique Binary Search Trees II
/// 
/// Given an integer `n`, return **all structurally unique BSTs** (binary search trees)
/// that store values from `1` to `n`.
///
/// **Example 1:**
/// ```plaintext
/// Input: n = 3
/// Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
/// Explanation: There are 5 unique BSTs.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: n = 1
/// Output: [[1]]
/// ```
///
/// **Constraints:**
/// - `1 <= n <= 8`
///
/// # Solution:
/// - **Time Complexity:** `O(C_n)` where `C_n` is the `n-th` Catalan number
/// - **Space Complexity:** `O(C_n)`

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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return Vec::new();
        }
        Self::generate_subtrees(1, n)
    }

    fn generate_subtrees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None]; 
        }

        let mut all_trees = Vec::new();

        for root_val in start..=end {
            let left_subtrees = Self::generate_subtrees(start, root_val - 1);
            let right_subtrees = Self::generate_subtrees(root_val + 1, end);

            for left in &left_subtrees {
                for right in &right_subtrees {
                    let root = Rc::new(RefCell::new(TreeNode {
                        val: root_val,
                        left: left.clone(),
                        right: right.clone(),
                    }));
                    all_trees.push(Some(root));
                }
            }
        }

        all_trees
    }
}