use precarious::UniquePtr;

fn main() {
    println!("Hello, world!");

    let ptr = {
        let mut ptr = UniquePtr::new(7);
        println!("inner scope: initialized to {}", ptr);
        ptr.set(4);
        println!("inner scope: changed to {}", ptr);
        ptr
    };
    println!("outer scope: immutably {}", ptr);

    let val = ptr.get();
    assert_eq!(val, 4);
    println!("{:#?}", ptr);

    lets_leak_memory();
}

fn lets_leak_memory() {
    let ptr = UniquePtr::new(45u32);
    std::mem::forget(ptr);
}

