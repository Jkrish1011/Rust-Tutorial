/*

Merge Two Sorted Lists
----------------------

You are given the heads of two sorted linked lists list1 and list2.
Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
Return the head of the merged linked list.

Example 1:
---------
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:
---------
Input: list1 = [], list2 = []
Output: []

Example 3:
---------
Input: list1 = [], list2 = [0]
Output: [0]

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
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
     let mut dummy: ListNode = ListNode::new(-1);
     let mut tail: &mut ListNode = &mut ListNode;

     while list1.is_some() && list2.is_some() {
      if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
        tail.next = list1.clone();
        list1 = list1.unwrap().next;
      } else {
        tail.next = list2.clone();
        list2 = list2.unwrap().next;
      }
      // tail is a reference, so when we unwrap a mutable, we get a box, which inturn is a reference. so it becomes a mutable reference
      tail = tail.next.as_mut().unwrap();
      
     }

     if list1.is_some() {
      tail.next = list1;
    }

    if list2.is_some() {
      tail.next = list2;
    }

    dummy.next
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn TestMergeLinkedList() {
        let mut l1: Option<Box<ListNode>> = Some(Box::new(ListNode{val: 1, next: Some(Box::new(ListNode{val: 2, next: None}))}));
        let mut l2: Option<Box<ListNode>> = Some(Box::new(ListNode{val: 3, next: Some(Box::new(ListNode{val: 4, next: None}))}));
        
        let res: Option<Box<ListNode>> = Solution::merge_two_lists(l1, l2);
        println!("{:?}", res);
    }
}