#![allow(dead_code)]

mod factorial;
mod monads;

use monads::Monad;



fn main() {
    benchmark_factorial(1_000_000);

    /*
    let a = Some(3);
    let b = a.attempt(add_one);

    if let Some(val) = b {
        println!("b = {val}");
    } else {
        println!("No value found for b");
    }
    */
}

fn add_one(a: i32) -> i32 {
    a + 1
}

fn benchmark_factorial(times: usize) {
    let i = 24;

    let elapsed_time = factorial::time(factorial::iter, i, times);
    println!("Iterator: {} ns", elapsed_time);

    let elapsed_time = factorial::time(factorial::looping, i, times);
    println!("Looping: {} ns", elapsed_time);

    let elapsed_time = factorial::time(factorial::recursive, i, times);
    println!("Recursive: {} ns", elapsed_time);

    let elapsed_time = factorial::time(factorial::recursive_with_tail, i, times);
    println!("Recursive with tail: {} ns", elapsed_time);
}
