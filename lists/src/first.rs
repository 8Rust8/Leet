use std::mem;

#[allow(unused)]
#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // below line says
        // take the head where self is pointing and replace it will an Link::Empty
        // match with what was taken away above. Point it we need to make sure Head is pointing somewhere
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // here we are re-point the head from Link::Empty to the next node inside the older
                // Head which we took at the beginning
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn drop_self(&mut self) {
        self.head = Link::Empty;
    }

    pub fn drop_all(&mut self) {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => println!("nothing left to drop"),
            Link::More(box_node) => {
                self.head = box_node.next;
                self.drop_all();
            }
        }
    }
}

pub trait Drop {
    fn drop(&mut self);
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

// uncomment below two lines for tests
// #[cfg(test)]
// mod tests;
