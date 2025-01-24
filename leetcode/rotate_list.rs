/// 
/// Problem: Rotate List
/// 
/// Given the head of a linked list, rotate the list to the right by `k` places.
///
/// **Example 1:**
/// ```plaintext
/// Input: head = [1,2,3,4,5], k = 2
/// Output: [4,5,1,2,3]
/// Explanation: Rotating right by 2 shifts the last two elements to the front.
/// ```
///
/// **Example 2:**
/// ```plaintext
/// Input: head = [0,1,2], k = 4
/// Output: [2,0,1]
/// Explanation: Rotating right by 4 is equivalent to rotating by 1 (since k = 4 % 3).
/// ```
///
/// **Constraints:**
/// - `0 <= length of list <= 500`
/// - `-100 <= Node.val <= 100`
/// - `0 <= k <= 2 * 10^9`
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() || k == 0 {
            return head;
        }

        
        let mut length = 1;
        let mut tail = head.as_ref().unwrap(); 
        while let Some(ref next) = tail.next {
            tail = next;
            length += 1;
        }

        let k = k as usize % length;
        if k == 0 {
            return head;
        }

        let mut head = head;
        let mut new_tail = head.as_mut().unwrap();
        for _ in 0..(length - k - 1) {
            new_tail = new_tail.next.as_mut().unwrap();
        }

        let mut new_head = new_tail.next.take();
        new_tail.next = None; 

        let mut temp = &mut new_head;
        while temp.as_mut().unwrap().next.is_some() {
            temp = &mut temp.as_mut().unwrap().next;
        }
        temp.as_mut().unwrap().next = head;

        new_head
    }
}