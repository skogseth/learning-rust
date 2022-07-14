fn main() {
    println!("Hello, world!");

    let mut a: i32 = 5;

    let a_ptr: *mut i32 = &mut a;
    let b_ptr: *mut i32 = a_ptr;

    unsafe {
        *a_ptr = 10;
        println!("value: {}", *b_ptr);
    }
}
