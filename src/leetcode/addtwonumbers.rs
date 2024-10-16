/*
You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order, and each of their nodes contains a single digit.
Add the two numbers and return the sum as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example 1:
----------
Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.

Example 2:
----------
Input: l1 = [0], l2 = [0]
Output: [0]

Example 3:
----------
Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]

Constraints:

The number of nodes in each linked list is in the range [1, 100].
0 <= Node.val <= 9
It is guaranteed that the list represents a number that does not have leading zeros.


*/

pub struct Solution;

// Definition for singly-linked list.
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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut increment = 0;
        let mut retNode = Some(Box::new(ListNode::new(-1)));
        let mut tmpNode = retNode.as_mut();
        
        // Taking mutable reference to the input parameters passed.
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

        while l1.is_some() || l2.is_some() || increment != 0 {
            if let Some(list1) = l1 {
                increment += list1.val;
                l1 = list1.next.as_ref();
            }
            if let Some(list2) = l2 {
                increment += list2.val;
                l2 = list2.next.as_ref();
            }
            let mut currValue = 0;
            currValue = increment % 10;
            increment = increment / 10;
            
            tmpNode.as_mut().unwrap().next = Some(Box::new(ListNode::new(currValue)));
            tmpNode = tmpNode.unwrap().next.as_mut();
        }
        return retNode.unwrap().next;
    }
}