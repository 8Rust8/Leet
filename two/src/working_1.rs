#![allow(unused)]
use std::mem;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

trait Solution {
    fn get_sum_carry(x1: &Box<ListNode>, x2: &Box<ListNode>, c: i32) -> (i32, i32);
    fn append_node(&mut self, num: i32);
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

    fn add_one(&mut self, new_node: Option<Box<ListNode>>) {
        self.next = new_node;
    }

    fn get_last_node(&mut self) -> &mut Box<ListNode> {
        let mut last_node = self.next.as_mut();
        while let Some(boxed_node) = last_node {
            last_node = boxed_node.next.as_mut();
        }

        last_node.unwrap()
    }

    fn get_val_vec(&self) -> Vec<i32> {
        let mut r: Vec<i32> = Vec::new();
        r.push(self.val.clone());
        let mut next_node = self.next.as_ref();
        while let Some(boxed_node) = next_node {
            r.push(next_node.as_ref().unwrap().val.clone());
            next_node = boxed_node.next.as_ref();
        }
        r
    }
}

impl Solution for ListNode {
    fn get_sum_carry(x1: &Box<ListNode>, x2: &Box<ListNode>, mut c: i32) -> (i32, i32) {
        let mut s = x1.val + x2.val + c;

        if s > 9 {
            s = s / 10;
            c = s % 10;
        } else {
            c = 0;
        }

       // let new_node = ListNode::new(s);

        (s, c)
    }

    fn append_node(&mut self, num: i32) {
        match &mut self.next {
            Some(next_node) => next_node.append_node(num),
            None => {
                self.next = Some(Box::new(ListNode::new(num)));
            }
        }
    }

    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut x1 = l1.unwrap();
        let mut x2 = l2.unwrap();
        let mut c = 0;
        let mut sum: ListNode = ListNode::new(0);
        
        while x1.next.is_some() || x2.next.is_some() {
            let sum_carry =  Self::get_sum_carry(&x1, &x2, c);
            println!("new_node :: {:?}", sum_carry.0);
            sum.append_node(sum_carry.0);
            // sum.next = Some(Box::new(sum_carry.0));
            c = sum_carry.1;
            x1 = x1.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
            x2 = x2.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
        }

        let sum_carry = Self::get_sum_carry(&x1, &x2, c);
        println!("new_node :: {:?}", sum_carry.0);
        sum.append_node(sum_carry.0);
        // sum.next.get_or_insert(Box::new(sum_carry.0));

        c = sum_carry.1;
        if c > 0 {
            let new_node = ListNode::new(c);
            println!("new_node :: {:?}", new_node);
            sum.append_node(sum_carry.0);
            //sum.next.get_or_insert(Box::new(new_node));
        }

        sum.next
    }

}

fn main() {
    let mut l1 = Some(Box::new(ListNode { val: 1, next: None }));
    let mut l2 = Some(Box::new(ListNode { val: 2, next: None }));
    let l3 = Some(Box::new(ListNode { val: 3, next: None }));

    l1.as_mut().map(|box_node| {
        box_node.add_one(l3.clone());
    });

    l2.as_mut().map(|box_node| {
        box_node.add_one(l3.clone());
    });

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
