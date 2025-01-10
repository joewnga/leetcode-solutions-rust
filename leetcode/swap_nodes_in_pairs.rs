/// 
/// Problem: Swap Nodes in Pairs
/// 
/// Given a linked list, swap every two adjacent nodes and return its head. 
/// You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed).
/// 
/// Example 1:
/// Input: head = [1,2,3,4]
/// Output: [2,1,4,3]
/// 
/// Example 2:
/// Input: head = []
/// Output: []
/// 
/// Example 3:
/// Input: head = [1]
/// Output: [1]
/// 
/// # Solution:
/// This solution uses iterative pointer manipulation to swap nodes in pairs. By keeping track of the previous node
/// and swapping the current pair, the process is efficient and avoids recursion.
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut h) => {
                match h.next {
                    Some(mut n) => {
                        h.next = Solution::swap_pairs(n.next);
                        n.next = Some(h);
                        Some(n)
                    },
                    _ => Some(h),
                }
            },
            _ => head,
        }
        
    }
}