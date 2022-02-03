# include-bytes-plus

[![Crates.io](https://img.shields.io/crates/v/include-bytes-plus.svg)](https://crates.io/crates/include-bytes-plus)
[![Documentation](https://docs.rs/include-bytes-plus/badge.svg)](https://docs.rs/crate/include-bytes-plus/)
[![Build](https://github.com/DoumanAsh/include-bytes-plus/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/include-bytes-plus/actions?query=workflow%3ARust)


Improved version of Rust's `include_bytes` macro that allows to reinterpret input as differently array.

# Usage:

```rust
use include_bytes_plus::include_bytes;

let bytes = include_bytes!("tests/include.in");
let bytes_u16 = include_bytes!("tests/include.in" as u16);

assert_eq!(bytes.len(), bytes_u16.len() * 2);
```
