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

// impl Solution {
//     pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut newListNode = ListNode::new();
//         let headNode = ListNode::new();

//         while list1.next != None {
//             while list2.next != None {
//                 if list1.val > list2.val {
//                     newListNode.next = list2;
//                     newListNode = list2;
//                     list2 = list2.next;
//                 }
                
//                 if list1.val < list2.val {
//                     newListNode.next = list1;
//                     newListNode = list1;
//                     list1 = list1.next;
//                 }

//                 if list1.val == list2.val {
//                     newListNode.next = list1;
//                     newListNode = list1;
//                     list1 = list1.next;
//                 }
//             }
//         }

//         return headNode
//     }
// }