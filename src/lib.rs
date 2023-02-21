//! A convenience macro for writing pattern-matching tests in the Rust programming language.
//!
//! The `assert_let` macro tests whether a value matches a given pattern, binding the pattern in the current scope if the match succeeds and causing a panic if the match //! fails.
//!
//! (Strongly inspired by [assert_matches](https://github.com/murarth/assert_matches))
//!
//! ```
//! use assert_let_bind::assert_let;
//!
//! #[derive(Debug)]
//! enum Foo {
//!     A(i32, i32),
//!     B(i32),
//! }
//!
//! let foo = Foo::A(3000, 2000);
//!
//! assert_let!(Foo::A(x, y) = foo);
//! assert_eq!(x + y, 5000);
//! ```

/// Asserts that a pattern matches a given expression.
///
/// Generally speaking, `assert_let(pattern, expr)` is roughly equivalent to:
///
/// ```rust
/// # #[cfg(FALSE)]
/// let pattern = expr else { panic!("some panic message with {} {}", pattern, expr)};
/// ```
#[macro_export]
macro_rules! assert_let {
    ( $pat:pat = $e:expr ) => {
        let expr = $e;
        #[rustfmt::skip]
        let $pat = expr else {
            panic!(
                "assertion failed: `{:?}` does not match `{}`",
                expr,
                stringify!($pat),
            );
        };
    };

    ( $pat:pat = $e:expr, $($arg:tt)* ) => {
        let expr = $e;
        #[rustfmt::skip]
        let $pat = expr else {
            panic!(
                "assertion failed: `{:?}` does not match `{}`: {}",
                expr,
                stringify!($pat),
                format_args!($($arg)*),
            );
        };
    };
}
