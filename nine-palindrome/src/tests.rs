use crate::nine_palindrome::is_palindrome;

#[test]
fn pass() {
    assert!(is_palindrome(22));
    assert!(is_palindrome(210012));
}
#[test]
#[should_panic]
fn fails() {
    assert!(is_palindrome(23));
    assert!(is_palindrome(10));
}
