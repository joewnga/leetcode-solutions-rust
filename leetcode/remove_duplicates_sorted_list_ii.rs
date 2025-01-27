/// 
/// Problem: Remove Duplicates from Sorted List II
/// 
/// Given the `head` of a sorted linked list, remove **all** nodes that have duplicate numbers, 
/// leaving only **distinct numbers** from the original list. Return the modified list.
///
/// **Example 1:**
/// ```plaintext
/// Input: head = [1,2,3,3,4,4,5]
/// Output: [1,2,5]
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: head = [1,1,1,2,3]
/// Output: [2,3]
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
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;

        while let Some(mut node) = prev.as_mut().unwrap().next.take() {
            let mut duplicate = false;

            while let Some(next_node) = node.next.as_mut() {
                if next_node.val == node.val {
                    duplicate = true;
                    node.next = next_node.next.take();
                } else {
                    break;
                }
            }

            if duplicate {
                prev.as_mut().unwrap().next = node.next;
            } else {
                prev.as_mut().unwrap().next = Some(node);
                prev = &mut prev.as_mut().unwrap().next;
            }
        }

        dummy.unwrap().next
    }
}