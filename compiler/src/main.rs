use std::ffi::c_int;

mod env;

use env::Os;

#[link(name = "math")]
extern {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main() {
    println!("Hello, world!");
    let os = Os::current();
    println!("Current os: {:?}", os);

    let a: c_int = 4;
    let b: c_int = 5;
    let c = unsafe { add(a,b) };
    println!("{a} + {b} = {c}");
}
