API:
```rust
pub fn log2fix(x: u32, precision: usize) -> i32
pub fn logefix(x: u32, precision: usize) -> i32
pub fn log10fix(x: u32, precision: usize) -> i32
```

You can play around with a cli example:
```shell
cargo run --example cli
```

Based on https://github.com/dmoulding/log2fix C implementation.