# Chapter 1

* assert marco

```rust,editable
fn inc(n: i32) -> i32 {
    n + 1
}
assert!(3, inc(2));
println!("{}", inc(5));
```