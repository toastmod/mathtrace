# mathtrace
A simple proc-macro to trace math calls.

# Example
```rust
use mathtrace::*;

#[mathtrace]
fn main() {
    let a = 1;
    let b = 2;
    let c = a + b;
    let d = 1 + c;
    let e = 6 + 5;
    let f = d + e;

    println!("Hello, world!");
}
```
Output:
```
a = 1
b = 2
c = a + b = (1) + (2)
d = 1 + c = (1) + (3)
e = 6 + 5 = (6) + (5)
f = d + e = (4) + (11)
Hello, world!
```
