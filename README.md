# Advanced math

## Description
This crate is created for advanced math operations.

## Usage

### Sum

```rust
use advanced_math::sum;

fn f(x: usize) -> usize {
    x
}

fn main() {
    let a = 1;
    let b = 2;
    let sum: usize = sum(a, b, f);
    
    println!("{} + {} = {}", a, b, sum);
}
```

This code execute a Σ sum with X=a and Σ going to b depending on f(x). So sum(0, 2, f(x)) with f(x) = x is 0 + 1 + 2 = 3.

### Limit

#### _Soon_