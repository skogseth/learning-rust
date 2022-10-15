use precarious::UniquePtr;

fn strtok<'a>(string: &mut &'a str, delimiter: char) -> &'a str {
    if let Some((prefix, suffix)) = string.split_once(delimiter) {
        *string = suffix;
        prefix
    } else {
        "" 
    }
}

fn main() {
    println!("Hello, world!");

    let mut str_1 = "Hello world";
    let str_2 = strtok(&mut str_1, ' ');
    println!("str_1 = {}", str_1);
    println!("str_2 = {}", str_2);
    
    let mut string = "Vous vous s'appellez comment?";
    loop {
        let word = strtok(&mut string, ' ');
        if word == "" {
            break;
        }
        println!("{}", word);
    }    

    let ptr = {
        let ptr = UniquePtr::new(7);
        println!("inner scope: val = {}", ptr.get());
        ptr
    };
    println!("outer scope: still {}", ptr.get());
}
