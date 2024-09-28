# Simple exercises

## Pattern Matching with Multiple Bindings and Guard Conditions in Rust

This project demonstrates how Rust handles pattern matching with multiple bindings and guard conditions.

## Overview

The code consists of a function `check` that always returns `false` after printing its argument, and a `checker` function that uses a tuple `(1, 2)` with a pattern match. The match attempts to bind both elements of the tuple to a variable `x` and evaluates them using the `check` function. Since the `check` function always returns `false`, the match falls through to the default case, which prints `"4"`.

### Code

```rust
fn check(x: i32) -> bool {
    println!("{}", x);
    false
}

fn checker() {
    match (1, 2) {
        (x, _) | (_, x) if check(x) => println!("3"),
        _ => println!("4"),
    }
}

fn main() {
    checker();
}
