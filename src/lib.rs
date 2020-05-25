#![no_std]

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
    fn from(t: T) -> YoloError {
        panic!("YOLO'd an error: {:?}", t)
    }
}

/**

```
#![feature(try_blocks)]
use ::yolo_block::yolo;
use ::std::convert::TryFrom;

let foo = yolo! {
  u32::try_from(5u64)?
};
```

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
