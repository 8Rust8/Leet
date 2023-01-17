#[allow(unused)]
#[derive(Debug)]

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

pub struct IntoIter<T>(List<T>);

//
impl <T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
            //next: mem::replace(&mut self.head, None),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // if head points to None then None goes to map and None gets returned
        match self.head {
            Some(_) => self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            }),
            None => None,
        }
    }

    pub fn peep(&self) -> Option<&T> {
        match self.head {
            Some(_) => self.head.as_ref().map(|node| &node.elem),
            None => None,
        }
    }

    // use this methong to map the output and change the refferencing value , see test
    pub fn peep_mut(&mut self) -> Option<&mut T> {
        match self.head {
            Some(_) => self.head.as_mut().map(|node| &mut node.elem),
            None => None,
        }
    }

    pub fn drop_self(&mut self) {
        self.head = None;
    }

    pub fn drop_all(&mut self) {
        match self.head.take() {
            None => println!("nothing left to drop"),
            Some(box_node) => {
                self.head = box_node.next;
                self.drop_all();
            }
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub trait Drop {
    fn drop(&mut self);
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        // `while let` == "do this thing until this pattern doesn't match"
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            //cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod tests;
