use mathtrace::*;

fn do_stuff(x: i32) -> i32 {
    x * 5
}

struct Thing {
    x: i32
}

fn thing(x: i32) -> Thing {
    Thing {x}
}

#[mathtrace]
fn main() {
    let mut a = 1;
    let b = 2;
    let c = a + b;
    let d = 1 + c;
    let e = 6 + 5;
    let f = d + e;
    let g = do_stuff(f);
    do_stuff(a) + 1;

    let h = thing(b);

    for i in 0..5 {
        a += 1;
    }

    println!("Hello, world!");
}
