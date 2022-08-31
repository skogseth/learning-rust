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

    let mut result = Ok(2);
    for _ in 0..4 {
        result = railway!{ decrement(result) };
        println!("{result:?}");
    }
}

fn decrement(val: usize) -> Result<usize, String> {
    if val > 0 {
        Ok(val - 1)
    } else {
        Err(String::from("Value too small to decrement!"))
    }
}