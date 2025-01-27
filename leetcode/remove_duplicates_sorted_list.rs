/// 
/// Problem: Remove Duplicates from Sorted List
/// 
/// Given the `head` of a sorted linked list, delete all duplicates such that each element appears only once.
/// 
/// Return the modified linked list.
///
/// **Example 1:**
/// ```plaintext
/// Input: head = [1,1,2]
/// Output: [1,2]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: head = [1,1,2,3,3]
/// Output: [1,2,3]
/// ```
///
/// **Constraints:**
/// - The number of nodes in the list is in the range `[0, 300]`.
/// - `-100 <= Node.val <= 100`
/// - The list is **sorted** in ascending order.
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current = dummy.as_mut().next.as_mut();

        while let Some(node) = current {
            while let Some(next_node) = node.next.as_mut() {
                if node.val == next_node.val {
                    node.next = next_node.next.take(); 
                } else {
                    break;
                }
            }
            current = node.next.as_mut(); 
        }

        dummy.next
    }
}