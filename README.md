# mathtrace
A simple proc-macro to trace math calls.

# Example
```rust
use mathtrace::*;

fn do_stuff(x: i32) -> i32 {
    x * 5
}

#[mathtrace]
fn main() {
    let a = 1;
    let b = 2;
    let c = a + b;
    let d = 1 + c;
    let e = 6 + 5;
    let f = d + e;
    let g = do_stuff(f);
    do_stuff(a) + 1;

    println!("Hello, world!");
}
```
Output:
```
b = 2
c = (1) + (2)
d = (1) + (3)
e = (6) + (5)
f = (4) + (11)
g = do_stuff(f) = 75
(do_stuff(a) = 5) + (1)
Hello, world!
```
