/*
Swap Nodes in Pairs
-------------------

Given a linked list, swap every two adjacent nodes and return its head. 
You must solve the problem without modifying the values in the list's nodes (i.e., only nodes themselves may be changed.)

Example 1:
-------------------
Input: head = [1,2,3,4]
Output: [2,1,4,3]


Example 2:
-------------------

Input: head = []
Output: []

Example 3:
-------------------

Input: head = [1]
Output: [1]

Example 4:
-------------------
Input: head = [1,2,3]
Output: [2,1,3]

Constraints:

The number of nodes in the list is in the range [0, 100].
0 <= Node.val <= 100

*/

// Definition for singly-linked list.
pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        } 
    }
}

impl Solution {
    pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode { val: 0, next: head };
        let mut prev = &mut head;

        while let Some(mut l1) = prev.next.take() {
            if let Some(mut l2) = l1.next.take() {
                l1.next = l2.as_mut().next.take();
                l2.next = Some(l1);
                prev.next = Some(l2);
                prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                prev.next = Some(l1);
                break;
            }x
        }
        head.next
    }

    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = ListNode { val: 0, next: head };
        let mut prev = &mut head;

 
        head.next
    }
}