# assert_let_bind

A convenience macro for writing pattern-matching tests in the Rust programming
language.

The `assert_let` macro tests whether a value matches a given pattern, binding
the pattern in the current scope if the match succeeds and causing a panic if
the match fails.

(Strongly inspired by
[assert_matches](https://github.com/murarth/assert_matches))

```
use assert_let_bind::assert_let;

#[derive(Debug)]
enum Foo {
    A(i32, i32),
    B(i32),
}

let foo = Foo::A(3000, 2000);

assert_let!(Foo::A(x, y), foo);
assert_eq!(x + y, 5000);
```

Generally speaking, `assert_let(pattern, expr)` is roughly equivalent to:

```rust
let pattern = expr else { panic!("some panic message with {} {}", pattern, expr)};
```

## Usage

This macro relies on `let else`, and thus only compiles from Rust 1.65 onwards.

To add it to your project:

```rust
cargo add assert_let_bind
```

To add it only to your tests:

```rust
cargo add --dev assert_let_bind
```

## License

This code is distributed under the terms of the MIT license. See LICENSE file
for details.

## Contributing

Contributions are unnecessary because this crate is already perfect.

In the unlikely eventuality that it isn't, issues and pull requests welcome.
