#![feature(try_blocks)]
#![no_implicit_prelude]
use ::std::convert::TryFrom;
use ::yolo_block::yolo;

#[test]
fn normal_enough() {
    let _foo: u8 = yolo! {
        u8::try_from(5u64)?
    };
}

#[test]
#[should_panic(expected = "YOLO'd an error: TryFromIntError")]
fn normal_enough_fail() {
    let _foo: u8 = yolo! {
        u8::try_from(500u64)?
    };
}

#[test]
#[should_panic(expected = "YOLO'd an error: 0")]
fn doesnt_implement_error() {
    let _foo: usize = yolo! {
        [].binary_search(&3)?
    };
}

#[test]
#[should_panic(expected = "YOLO'd an error: NoneError")]
fn option() {
    let _foo: u32 = yolo! {
        ::std::option::Option::None?
    };
}
