use crate::second::{Drop, List};

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
    assert_eq!(list.peep(), Some(& 2));
    assert_eq!(list.peep_mut(), Some(&mut 2));
}

#[test]
fn pass() {
    let mut list = List::new();
    assert_eq!(list.pop(), None);
    assert_eq!(list.peep(), None);
    assert_eq!(list.peep_mut(), None);
    list.push(1);
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
fn peep() {
    let mut list = List::new();
    list.push(3);
    assert_eq!(list.peep(), Some(&3));
}

#[test]
fn peep_mut() {
    let mut list = List::new();
    list.push(3);
    list.peep_mut().map(|elem_ref| {
        *elem_ref = 32;
    });
    assert_eq!(list.peep_mut(), Some(&mut 32));
    assert_eq!(list.pop(), Some(32));
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push("one".to_string()); list.push("two".to_string()); list.push("three".to_string());
    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some("three".to_owned()));
    assert_eq!(iter.next(), Some("two".to_owned()));
    assert_eq!(iter.next(), Some("one".to_owned()));
    assert_eq!(iter.next(), None);
}

// simmilar to fn drop() you can test drop_all() and stop_self()
