/// 
/// Problem: You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
/// 
/// Example 1:
/// 
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
/// 
/// 
/// 
/// Solution 1:
/// Time complexity: O(max(m,n))
/// Space complexity: O(max(m,n))
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n), 
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next)
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(Solution::add_two_numbers(carry, n1.next), n2.next)
                    }))
                }
            }
        }
    }
}









/// Solution 2:
/// Time complexity: O(max(m,n))
/// Space complexity: O(max(m,n))

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>, 
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut p = l1; 
        let mut q = l2; 
        let mut carry = 0; 
        let mut dummy_head = Some(Box::new(ListNode::new(0))); 
        let mut current = &mut dummy_head; 
        
        while p.is_some() || q.is_some() {
            let x = p.as_ref().map_or(0, |node| node.val); 
            let y = q.as_ref().map_or(0, |node| node.val); 
            let sum = x + y + carry; 
            
            carry = sum / 10; 
            let new_node = Some(Box::new(ListNode::new(sum % 10))); 
            
            current.as_mut().unwrap().next = new_node; 
            current = &mut current.as_mut().unwrap().next;
            
            
            p = p.and_then(|node| node.next);
            q = q.and_then(|node| node.next);
        }
        
        if carry > 0 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry))); 
        }
        
        dummy_head.unwrap().next 
    }
}