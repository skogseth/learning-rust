use std::io;

mod blackjack;

fn main() {
    println!("Welcome to the blackjack table");
    while play_round() {
        blackjack::play();
    }
}

fn play_round() -> bool {
    loop {
        println!("Do you want to play? (yes/no)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        println!("");

        if input == "yes" {
            return true
        } else if input == "no" {
            return false
        } else {
            println!("Invalid answer. Your input was: {}.\n", input);
            continue;
        }
    }
}
