#![allow(unused)]
use std::mem;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

trait Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn push(&mut self, val: i32) {
        let new_node = ListNode::new(val);
        self.next = Some(Box::new(new_node));
    }
}

impl Solution for ListNode {
   fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let x = l1.as_ref().map(|box_node| box_node.val).unwrap_or(0);
        let y = l2.as_ref().map(|box_node| box_node.val).unwrap_or(0);
        Some(Box::new(ListNode { val: x+y, next: None }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let l1 = Some(Box::new(ListNode { val: 1, next: None }));
        let l2 = Some(Box::new(ListNode { val: 1, next: None }));
        let l3: Option<Box<ListNode>>= None;
        let l4: Option<Box<ListNode>> = None;
        let l5: Option<Box<ListNode>>= Some(Box::new(ListNode { val: 1, next: None }));;
       // let mut result = ListNode::new(3);
        let r = ListNode::add_two_numbers(l1, l2) ;
        assert_eq!(r, Some(Box::new(ListNode { val: 2, next: None })));

        let r = ListNode::add_two_numbers(l3,l4) ;
        assert_eq!(r, Some(Box::new(ListNode { val: 0, next: None })));

        let r = ListNode::add_two_numbers(None, l5) ;
        assert_eq!(r, Some(Box::new(ListNode { val: 1, next: None })));
    }
}
