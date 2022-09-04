fn main() {
    println!("Hello, world!");
    let a = 6;
    let b = 4;
    let c = unsafe { cmath::multiply(a, b) };
    println!("{} * {} = {}", a, b, c);

    let d = 36.;
    let e = unsafe { auto::sqrt(d) };
    println!("sqrt({}) = {}", d, e);
}
