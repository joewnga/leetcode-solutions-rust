/// 
/// Problem: Merge k Sorted Lists
/// 
/// You are given an array of `k` linked lists, each sorted in ascending order. Merge all the linked lists into one sorted linked list and return it.
/// 
/// Example 1:
/// Input: lists = [[1,4,5],[1,3,4],[2,6]]
/// Output: [1,1,2,3,4,4,5,6]
/// Explanation: The linked lists are:
/// [
///   1 -> 4 -> 5,
///   1 -> 3 -> 4,
///   2 -> 6
/// ]
/// Merging them into one sorted list:
/// 1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6
/// 
/// Example 2:
/// Input: lists = []
/// Output: []
/// 
/// Example 3:
/// Input: lists = [[]]
/// Output: []
/// 
/// # Solution:
/// 
/// Time complexity: O(N * log(k))
/// Space complexity: O(k)


use std::cmp::{Reverse, Ordering};
use std::collections::BinaryHeap;

type NodeOpt = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_k_lists(lists: Vec<NodeOpt>) -> NodeOpt {
        let mut head = None;
        let mut tail = &mut head;
        let mut heap = BinaryHeap::from(lists);

        while let Some(Some(mut curr)) = heap.pop() {
            if curr.next.is_some() {
                heap.push(curr.next.take());
            }
            tail = &mut tail.insert(curr).next;
        }
        head
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.val).cmp(&Reverse(other.val))
    }
}