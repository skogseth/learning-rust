fn main() {
    println!("Hello, world!");
    let a = 6;
    let b = 4;
    let c = unsafe { cmath::multiply(a, b) };
    println!("Hello!");
    println!("{} * {} = {}", a, b, c);
}
