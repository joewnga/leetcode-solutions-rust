/// 
/// Problem: Merge Two Sorted Lists
/// 
/// You are given the heads of two sorted linked lists `list1` and `list2`. Merge the two lists into one sorted list.
/// The list should be made by splicing together the nodes of the first two lists.
/// 
/// Return the head of the merged linked list.
/// 
/// Example 1:
/// Input: list1 = [1,2,4], list2 = [1,3,4]
/// Output: [1,1,2,3,4,4]
/// 
/// Example 2:
/// Input: list1 = [], list2 = []
/// Output: []
/// 
/// Example 3:
/// Input: list1 = [], list2 = [0]
/// Output: [0]
/// 
/// # Solution:
/// 
/// Time complexity: O(n + m)
/// Space complexity: O(n + m)

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Solution::merge_two_lists(l1.next.take(), Some(l2));
                    Some(l1)
                } else {
                    l2.next = Solution::merge_two_lists(Some(l1), l2.next.take());
                    Some(l2)
                }
            }
        }
    }
}
