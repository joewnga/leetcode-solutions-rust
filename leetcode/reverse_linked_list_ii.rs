/// 
/// Problem: Reverse Linked List II
/// 
/// Given the `head` of a singly linked list and two integers `left` and `right`, reverse the nodes 
/// of the list **from position `left` to `right`**, and return the modified linked list.
///
/// **Example 1:**
/// ```plaintext
/// Input: head = [1,2,3,4,5], left = 2, right = 4
/// Output: [1,4,3,2,5]
/// Explanation: The sublist `[2,3,4]` is reversed.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: head = [5], left = 1, right = 1
/// Output: [5]
/// ```
///
/// **Constraints:**
/// - The number of nodes in the list is in the range `[1, 500]`.
/// - `-500 <= Node.val <= 500`
/// - `1 <= left <= right <= n`
///
/// # Solution:
/// - **Time Complexity:** `O(n)`
/// - **Space Complexity:** `O(n)`

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
use std::iter::successors;
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut values = successors(head.as_ref(), |node| node.next.as_ref())
            .map(|node| node.val)
            .collect::<Vec<_>>();

       
        values[left as usize - 1..right as usize].reverse();

       
        let mut dummy_head = ListNode::new(-1);
        values.into_iter().fold(&mut dummy_head, |current_node, value| {
            current_node.next = Some(Box::new(ListNode::new(value)));
            current_node.next.as_mut().unwrap()
        });

        dummy_head.next
    }
}