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
    let mut a = 1;
    let b = 2;
    let c = a + b;
    let d = 1 + c;
    let e = 6 + 5;
    let f = d + e;

    do_stuff(a) + 1;

    for i in 0..5 {
        a += 1;
    }

    println!("Hello, world!");
}
```

Output:

```
mut a = 1
b = 2
c = (1) + (2)
d = (1) + (3)
e = (6) + (5)
f = (4) + (11)
(do_stuff(a)) + (1)
(a) += (1)
(a) += (1)
(a) += (1)
(a) += (1)
(a) += (1)
Hello, world!
```
