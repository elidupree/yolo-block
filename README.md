# yolo-block

Like a `try` block, but automatically unwraps the result. Effectively, inside a `yolo!` block, the `?` operator functions as `unwrap()`.

```rust
#![feature(try_blocks)]

let result = yolo! {
    "1".parse::<i32>()?
        + "2".parse::<i32>()?
        + "3".parse::<i32>()?
};
assert_eq!(result, 6);

// Panics with message "YOLO'd an error: ParseIntError { kind: InvalidDigit }"
let result = yolo! {
    "1".parse::<i32>()?
        + "foo".parse::<i32>()?
        + "3".parse::<i32>()?
};
```

## Motivation

This crate was primarily written as a joke, but I can imagine a practical use for it. While prototyping, you may want to write code that handles errors using the `?` operator, even if you haven't yet implemented a way to handle the errors. In this case, wrapping the code in a `yolo!` block allows you write code that correctly uses `?`, rather than leaving `unwrap()` calls that you might forget to replace later.

## Design and features

Since the `yolo!` macro uses a `try` block internally, it requires you to enable `#![feature(try_blocks)]` in the crate where you use it. As of this writing (Rust 1.43.1), `try` blocks are an unstable feature, so `yolo-block` is only available on nightly.

`yolo-block` is fully compatible with `#![no_std]` (and `#![no_implicit_prelude]`).

A `yolo!` block can handle *any* error type that implements Debug, even ones that don't implement Error.

Internally, to handle disparate types without having to construct a `Box<dyn Debug>`, we use a custom error type that can be converted from anything that implements Debug. The custom type is actually uninhabited, and the `From` conversion immediately panics.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
