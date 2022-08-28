use macros::add;

fn main() {
    println!("Hello, world!");

    let a = add!(1, 2, 3, 4);
    let b = add!(2);
    let c = add!();
    
    println!("a = {a}");
    println!("a = {b}");
    println!("a = {c}");
}

