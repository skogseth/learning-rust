use std::time::Instant;

pub fn iter(n: u128) -> u128 {
    (1..n + 1).product()
}

pub fn looping(n: u128) -> u128 {
    let mut acc = n;
    for i in 1..n {
        acc *= i;
    }
    acc
}

pub fn recursive(n: u128) -> u128 {
    if n == 0 {
        1
    } else {
        n * recursive(n - 1)
    }
}

pub fn recursive_with_tail(n: u128) -> u128 {
    tail(1, n)
}

fn tail(acc: u128, n: u128) -> u128 {
    if n == 0 {
        acc
    } else {
        tail(n * acc, n - 1)
    }
}

pub fn time<F: Fn(u128) -> u128>(f: F, n: u128, times: usize) -> u128 {
    let mut total_time = 0;
    for _ in 0..times {
        let now = Instant::now();
        f(n);
        total_time += now.elapsed().as_nanos();
    }
    total_time / times as u128
}
