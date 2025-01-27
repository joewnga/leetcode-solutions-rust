/// 
/// Problem: Partition List
/// 
/// Given the `head` of a linked list and a value `x`, partition it such that:
/// - All nodes **less than** `x` appear **before** nodes greater than or equal to `x`.
/// - Maintain the **original relative order** of nodes.
///
/// **Example 1:**
/// ```plaintext
/// Input: head = [1,4,3,2,5,2], x = 3
/// Output: [1,2,2,4,3,5]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: head = [2,1], x = 2
/// Output: [1,2]
/// ```
///
/// **Constraints:**
/// - The number of nodes in the list is in the range `[0, 200]`.
/// - `-100 <= Node.val <= 100`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(1)`
/// 
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left_dummy = Box::new(ListNode { val: 0, next: None });
        let mut right_dummy = Box::new(ListNode { val: 0, next: None });

        let (mut left, mut right) = (&mut left_dummy, &mut right_dummy);
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take(); 

            if node.val < x {
                left.next = Some(node);
                left = left.next.as_mut().unwrap();
            } else {
                right.next = Some(node);
                right = right.next.as_mut().unwrap();
            }
        }

        left.next = right_dummy.next;
        left_dummy.next
    }
}