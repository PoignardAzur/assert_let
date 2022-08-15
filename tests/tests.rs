#![allow(dead_code)]
#![allow(unstable_features)]
#![feature(let_else)]

use assert_let_bind::assert_let;

#[test]
fn test_assert_succeed() {
    let my_string = String::from("hello");

    assert_let!(Some(_), Some(42));
    assert_let!(Some(42), Some(42));
    assert_let!(Some(_), Some(my_string.clone()));
    assert_let!(Some(_string), Some(my_string.clone()));
    assert_let!(Some(the_string), Some(my_string));

    assert_let!([_a, _b, _c], [1, 2, 3]);

    dbg!(the_string);

    assert_let!(Some(_), Some(42), "some assert message");
    assert_let!(Some(_), Some(42), "some assert {}", "message");
}

#[derive(Debug)]
enum Foo {
    A(i32, i32),
    B(i32),
}

#[test]
fn test_complex_match() {
    let foo = Foo::A(3000, 2000);

    assert_let!(Foo::A(x, y), foo);
    assert_eq!(x + y, 5000);
}

#[test]
#[should_panic(expected = "assertion failed: `None` does not match `Some(42)`")]
fn test_assert_panic() {
    assert_let!(Some(42), None);
}

#[test]
#[should_panic(
    expected = "assertion failed: `None` does not match `Some(42)`: additional info (123)"
)]
fn test_assert_panic_with_message() {
    assert_let!(Some(42), None, "additional info ({})", 123);
}
