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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut headNode: ListNode = ListNode::new(0);
        let mut rangeNode: ListNode = ListNode::new(0);
        let mut l1_tmp: ListNode = ListNode::new(0);
        let mut l2_tmp: ListNode = ListNode::new(0);
        let mut l1_flag: bool = false;
        let mut l2_flag: bool = false;
        loop {
            match list1 {
                Some(l1) => {
                    l1_flag = true;
                    l1_tmp = *l1;
                },
                None => l1_flag = false
            }
            match list2 {
                Some(l2) => {
                    l2_flag = true;
                    l2_tmp = *l2;
                },
                None => l2_flag = false
            }
            
            if l1_flag == true && l2_flag == true {
                if l1.val > l2.val {

                }
            }
            break;
        }
        
        return Some(Box::new(headNode));
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn TestMergeLinkedList() {
        let mut l1: ListNode = ListNode::new(1);
        let mut l2: ListNode = ListNode::new(2);
        let res: Option<Box<ListNode>> = Solution::merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
        println!("{:?}", res);
    }
}