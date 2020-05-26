# yolo-block

Rust library providing `yolo!` blocks, which are like `try` blocks, except that they automatically unwrap the result. Effectively, inside a `yolo!` block, the `?` operator functions as `unwrap()`.

```rust
#![feature(try_blocks)]
use yolo_block::yolo;
use std::convert::TryFrom;

let result = yolo! {
    "1".parse::<usize>()?
        + usize::try_from(2i32)?
        + [0,1,2,3].binary_search(&3)?
};
assert_eq!(result, 6);

// Panics with the message "YOLO'd an error: ParseIntError { kind: InvalidDigit }"
let result = yolo! {
    "1".parse::<usize>()?
        + "foo".parse::<usize>()?
        + "3".parse::<usize>()?
};
```

## Motivation

This crate was primarily written as a joke, but I can imagine a practical use for it. While prototyping, you may want to write code that handles errors using the `?` operator, even if you haven't yet implemented a way to handle the errors. In this case, wrapping the code in a `yolo!` block allows you write code that correctly uses `?`, rather than leaving `unwrap()` calls that you might forget to replace later.

## Design and features

Since the `yolo!` macro uses a `try` block internally, it requires you to enable `#![feature(try_blocks)]` in the crate where you use it. As of this writing (Rust 1.43.1), `try` blocks are an unstable feature, so this crate is only available on nightly.

`yolo-block` is fully compatible with `#![no_std]` (and `#![no_implicit_prelude]`).

A single `yolo!` block can handle multiple error types. Those error types can be any type that implements Debug. No extra type annotations are needed.

Internally, we use a custom error type that can be converted from anything that implements Debug. In order to handle disparate types without having to construct a `Box<dyn Debug>`, the `From` conversion simply panics immediately, and the custom error type is actually uninhabited.

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
