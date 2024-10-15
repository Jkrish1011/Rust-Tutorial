#[derive(Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode {
            val: val,
            next: None,
        }
    }
}

#[cfg(test)]
pub mod tests{
    use super::*;

    fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
        let mut cur = None;
        for &value in vector.iter().rev() {
            let mut new_node = ListNode::new(value);
            new_node.next = cur;
            cur = Some(Box::new(new_node));
        }
        cur
    }

    #[test]
    fn TestLinkedList() {
        // let node = ListNode{val:0, next: Some(Box::new(None))}
        let vector = vec![0,1,2,3,4];
        println!("{:?}", to_list(vector));
    }
}