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
        match &mut self.head {
            Link::Empty => None,
            Link::More(node) => {
                let elem = Some(node.elem);
                // here is the trick
                self.head = mem::replace(&mut node.next, Link::Empty );
                //node.next = mem::replace(&mut node.next, Link::Empty);
                return elem;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        let (one , two )= (1,2);
        
        list.push(one);
        list.push(two);

        assert_eq!(list.pop(), Some(two));
        assert_eq!(list.pop(), Some(one));
        assert_eq!(list.pop(), None);
    }
}
