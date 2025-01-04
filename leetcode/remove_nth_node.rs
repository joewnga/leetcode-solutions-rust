/// 
/// Problem: Remove Nth Node From End of List
/// 
/// Given the `head` of a linked list, remove the nth node from the end of the list and return its head.
/// 
/// Example 1:
/// Input: head = [1,2,3,4,5], n = 2
/// Output: [1,2,3,5]
/// Explanation: The second node from the end is 4, so we remove it, resulting in [1,2,3,5].
/// 
/// Example 2:
/// Input: head = [1], n = 1
/// Output: []
/// Explanation: There's only one node, which is removed.
/// 
/// Example 3:
/// Input: head = [1,2], n = 1
/// Output: [1]
/// Explanation: The last node is removed.
/// 
/// # Solution:
/// 
/// Time complexity: O(n)
/// Space complexity: O(1)

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let cnt = std::iter::successors(head.as_ref(), |last| last.next.as_ref()).count();
        let mut dummy = Some(Box::new(ListNode{val: 0, next: head}));
        let mut prev = (0..cnt - n as usize).fold(dummy.as_mut(), |curr, _| curr.unwrap().next.as_mut() );
        prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next
        
    }
}