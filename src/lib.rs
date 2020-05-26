/*!

Like a `try` block, but automatically unwraps the result. Effectively, inside a `yolo!` block, the `?` operator functions as `unwrap()`.

Since the `yolo!` macro uses a `try` block internally, it requires you to enable `#![feature(try_blocks)]` in the crate where you use it. As of this writing (Rust 1.43.1), `try` blocks are an unstable feature, so this crate is only available on nightly.

A `yolo!` block can handle any error type that implements Debug, even ones that don't implement Error.

# Examples

```
#![feature(try_blocks)]
use yolo_block::yolo;

let result = yolo! {
    "1".parse::<i32>()?
        + "2".parse::<i32>()?
        + "3".parse::<i32>()?
};
assert_eq!(result, 6);
```

```should_panic
# #![feature(try_blocks)]
# use yolo_block::yolo;
// Panics with the message "YOLO'd an error: ParseIntError { kind: InvalidDigit }"
let result = yolo! {
    "1".parse::<i32>()?
        + "foo".parse::<i32>()?
        + "3".parse::<i32>()?
};
```

**/

#![feature(track_caller)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/yolo-block/0.1.0")]

use core::convert::From;
use core::fmt::Debug;

// This type might be useful for users in the future -
// so you could write a function that returns `Result<T, YoloError>` -
// but it can't implement Error, and without that trait,
// you'd run into gotchas if you tried to use ? on THAT result
// in an outer function that returned Result<U, Box<dyn Error>>.
//
// Right now, it can't implement Debug, and hence can't implement Error,
// because of a conflict with the blanket impl of From<T> for T.
// Even #![feature(specialization)] can't resolve this.
//
// Given the gotchas, we're not making this available in the public API
// until/unless we find a cleaner approach.
#[doc(hidden)]
pub enum YoloError {}

impl<T: Debug> From<T> for YoloError {
    // same rationale for these attributes as for unwrap_failed() from core/result.rs
    #[inline(never)]
    #[cold]
    #[track_caller]
    fn from(t: T) -> YoloError {
        panic!("YOLO'd an error: {:?}", t)
    }
}

/**

The macro to create a `yolo!` block.

See the [crate level docs](../yolo_block/index.html) for details.

**/

#[macro_export]
macro_rules! yolo {
  ($($contents:tt)*) => {{
    let result: ::core::result::Result<_, $crate::YoloError> = try {
      $($contents)*
    };
    // Note: The second match arm won't be needed here once the
    // compiler gets smarter about uninhabited types.
    match result {
      ::core::result::Result::Ok(r) => r,
      ::core::result::Result::Err(e) => match e {},
    }
  }}
}
