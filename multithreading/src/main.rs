use std::thread;
use std::time::Duration;

fn main() {
    let mut vec: Vec<_> = (1..10).collect();
    let handle = thread::spawn(move || {
        for _ in 1..10 {
            println!("hi number {} from the spawned thread!", vec.pop().unwrap());
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
