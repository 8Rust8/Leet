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
}

impl Solution for ListNode {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut x1 = l1.as_ref();
        let mut x2 = l2.as_ref();
        let mut c = 0;
        let mut sum: ListNode = ListNode::new(0);
        let mut curr = &mut sum;

        while x1.is_some() || x2.is_some() {
            let a = x1.map_or(0, |node| node.val);
            let b = x2.map_or(0, |node| node.val);
            let mut s = a + b + c;
            if s > 9 {
                c = s / 10;
                s = s % 10;
            } else {
                c = 0;
            }
            curr.next = Some(Box::new(ListNode::new(s)));
            curr = curr.next.as_mut().unwrap();

            x1 = x1.and_then(|f| f.next.as_ref());
            x2 = x2.and_then(|f| f.next.as_ref());
        }
        
        if c > 0 {
            curr.next = Some(Box::new(ListNode::new(c)));
        }

        sum.next
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    println!("Sum :: {:?}", ListNode::add_two_numbers(l1, l2));
}
