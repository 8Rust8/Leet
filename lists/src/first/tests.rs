use crate::first::{List, Drop};


#[test]
fn basics() {
    let mut list = List::new();
    let (one, two) = (1, 2);
    list.push(one);
    list.push(two);

    assert_eq!(list.pop(), Some(two));
    assert_eq!(list.pop(), Some(one));
}
#[test]
#[should_panic]
fn fails() {
    let mut list = List::new();
    assert_eq!(list.pop(), Some(2));
}
#[test]
fn drop() {
    let mut list = List::new();
    let one = 1;
    list.push(one);

    list.drop();

    assert_eq!(list.pop(), None);
}

#[test]
fn drop_self() {
    let mut list = List::new();
    let two = 2;
    list.push(two);

    list.drop_self();

    assert_eq!(list.pop(), None);
}

#[test]
fn drop_all() {
    let mut list = List::new();
    let (one, two) = (1, 2);
    list.push(one);
    list.push(two);

    list.drop_all();

    assert_eq!(list.pop(), None);
}
