# range-checked

[![Continuous integration](https://github.com/EricLBuehler/range-checked/actions/workflows/ci.yml/badge.svg)](https://github.com/EricLBuehler/range-checked/actions/workflows/ci.yml)

`range-checked` provides range checked types which leverage Rust's type system.

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