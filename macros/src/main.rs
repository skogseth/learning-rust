use macros::*;

fn main() {
    println!("Hello, world!");

    let a = add!(1, 2, 3);
    let b = add!(2);
    // let c = add!(); // compilation error

    println!("a = {a}");
    println!("a = {b}");

    let d = add!(8, 16; u8);
    println!("d = {d}");
}

