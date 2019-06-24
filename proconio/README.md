# proconio

[![crates.io](https://img.shields.io/crates/v/proconio.svg)](https://crates.io/crates/proconio)
[![docs.rs](https://docs.rs/proconio/badge.svg)](https://docs.rs/proconio)

Easy IO library for competitive programming.

`proconio` provides an easy way to read values from stdin (or other source).  The main is `input!` macro.

The macro's user interface is basically the same with [tanakh's input macro](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8).

```rust
use proconio::input;

input! {
    n: u8,
    m: u32,
    l: i32,
}

// now you can use n, m and l as variable.
println!("{} {} {}", n, m, l);
```

For more details, see documentation.
