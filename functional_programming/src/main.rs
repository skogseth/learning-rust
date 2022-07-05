use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let mut heavy = Cacher::new(|num: i32| -> f64 {
        println!("performing heavy calculation on {}...", num);
        thread::sleep(Duration::from_secs(2));
        3.1415
    });

    println!("{}", heavy.result(3));
    println!("{}", heavy.result(3));

    let i: i32 = 5;
    let u: u32 = 5u32;
    println!("{}", heavy.result(5));
    println!("{}", heavy.result(i));
    println!("{}", heavy.result(u as i32));

    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum();
    println!("Vec: {:?} -> sum = {}", v1, total);

    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();
    println!("Vec: {:?}", v2);
}

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Copy + Eq + Hash,
    V: Copy,
{
    calculation: T,
    results: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Copy + Eq + Hash,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            results: HashMap::new(),
        }
    }

    fn result(&mut self, arg: U) -> V {
        match self.results.get(&arg) {
            Some(&val) => val,
            None => {
                let val = (self.calculation)(arg);
                self.results.insert(arg, val);
                val
            }
        }
    }
}
