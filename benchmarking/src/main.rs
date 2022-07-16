mod factorial;

fn main() {
    let times = 1_000_000;
    benchmark_factorial(times);
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
