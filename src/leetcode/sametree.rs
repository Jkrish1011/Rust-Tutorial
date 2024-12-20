/*
Given the roots of two binary trees p and q, write a function to check if they are the same or not.

Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

Example 1:

Input: p = [1,2,3], q = [1,2,3]
Output: true

Example 2:

Input: p = [1,2], q = [1,null,2]
Output: false

Example 3:

Input: p = [1,2,1], q = [1,1,2]
Output: false

*/
use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}


impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {

        match (p, q) => {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,

            (Some(p_node), Some(q_node)) {
                let p_borrowed = p_node.borrow();
                let q_borrowed = q_node.borrow();

                p_borrowed.val == q_borrowed.val && 
                Self::is_same_tree(p_borrowed.left.clone(), q_borrowed.left.clone()) &&
                Self::is_same_tree(p_borrowed.right.clone(), q_borrowed.right.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_tree() {
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);

        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node2.left = Some(Rc::new(RefCell::new(node3)));
        node2.right = Some(Rc::new(RefCell::new(node4)));

        root.left = Some(Rc::new(RefCell::new(node5)));
        root.right = Some(Rc::new(RefCell::new(node2)));

        Solution::is_same_tree(Some(Rc::new(RefCell::new(root))), Some(Rc::new(RefCell::new(root))));
    }
}