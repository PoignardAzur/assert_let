#[macro_export]
macro_rules! assert_let {
    ( $pat:pat, $e:expr ) => {
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

    ( $pat:pat, $e:expr, $($arg:tt)* ) => {
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
