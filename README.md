# range-checked

[![Continuous integration](https://github.com/EricLBuehler/range-checked/actions/workflows/ci.yml/badge.svg)](https://github.com/EricLBuehler/range-checked/actions/workflows/ci.yml)
[![Documentation](https://github.com/EricLBuehler/range-checked/actions/workflows/docs.yml/badge.svg)](https://ericlbuehler.github.io/range-checked/range_checked/)

`range-checked` provides range checked types which leverage Rust's type system. Please see docs [here](https://ericlbuehler.github.io/range-checked/range_checked/).

## Example

```rust
use range_checked::I32Bounded;

fn main() {
    // Panics
    let _: I32Bounded<0, 128, false> = 128.try_into().unwrap(); 
    // Successful
    let _: I32Bounded<0, 128, true> = 128.try_into().unwrap(); 
    // Successful
    let _: I32Bounded<0, 128, false> = 64.try_into().unwrap(); 
}
```