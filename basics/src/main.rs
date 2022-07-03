fn main() {
    let simple_tuple = (64, false);
    println!("{:?}", swap(simple_tuple));

    const N: u32 = 10;
    println!("The {N}th fibonacci number is {}", fibonacci(N));

    let s: String = String::from("Hello my name is Herman I'm a Rustacean");

    let v = split(&s);

    for element in v {
        println!("{}", element);
    }
}

fn swap(tup: (i32, bool)) -> (bool, i32) {
    (tup.1, tup.0)
}

fn fibonacci(n: u32) -> u64 {
    let mut f_p: u64 = 0;
    let mut f_i: u64 = 1;
    let mut temp;

    for _i in 2..n {
        temp = f_i;
        f_i += f_p;
        f_p = temp;
    }

    f_i
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn split(s: &str) -> Vec<&str> {
    let mut i: usize = 0;
    let mut v: Vec<&str> = Vec::new();

    while i < s.len() {
        let s_i = first_word(&s[i..]);
        v.push(s_i);
        i += s_i.len() + 1;
    }

    v
}
