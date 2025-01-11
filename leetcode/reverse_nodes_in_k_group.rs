/// 
/// Problem: Reverse Nodes in k-Group
/// 
/// Given the head of a linked list, reverse the nodes of the list `k` at a time, and return the modified list.
/// 
/// k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not
/// a multiple of k, the remaining nodes are left as is.
/// 
/// Example 1:
/// Input: head = [1,2,3,4,5], k = 2
/// Output: [2,1,4,3,5]
/// 
/// Example 2:
/// Input: head = [1,2,3,4,5], k = 3
/// Output: [3,2,1,4,5]
/// 
/// # Solution:


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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut node = &mut head;
        for _ in 0..k {
            if let Some(n) = node {
                node = &mut n.next;
            } else {
                return head;
            }
        }

        
        let mut prev = Self::reverse_k_group(node.take(), k);


        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }



        prev
    }
}