use include_bytes_plus::include_bytes;

use std::fs::read;
use core::slice;

#[test]
fn should_include_u8_bytes() {
    let included = include_bytes!("tests/include.in");
    let expected = read("tests/include.in").expect("To read source code");
    assert!(included == expected.as_slice(), "included doesn't match expected file output");
}

#[test]
fn should_include_u16() {
    let included = include_bytes!("tests/include.in" as u16);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u16, expected.len() / 2)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u32() {
    let included = include_bytes!("tests/include.in" as u32);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u32, expected.len() / 4)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u64() {
    let included = include_bytes!("tests/include.in" as u64);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u64, expected.len() / 8)
    };
    assert!(included == expected, "included doesn't match expected file output");
}

#[test]
fn should_include_u128() {
    let included = include_bytes!("tests/include.in" as u128);
    let expected = read("tests/include.in").expect("To read source code");
    let expected = unsafe {
        slice::from_raw_parts(expected.as_ptr() as *const u128, expected.len() / 16)
    };
    assert!(included == expected, "included doesn't match expected file output");
}
