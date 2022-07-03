use std::io;
use cards::CardDeck;

mod player;
use player::Player;

pub fn play() {
    let mut deck = CardDeck::shuffled();
    let mut dealer = Player::initialize();
    let mut player = Player::initialize();

    player.draw(&mut deck);
    dealer.draw(&mut deck);
    player.draw(&mut deck);
    dealer.draw(&mut deck);

    println!("Dealers first card is:");
    dealer.show_first_card();
    println!("");

    while dealer.active || player.active {
        if player.active {
            if player.sum() >= 21 {
                player.active = false;
            } else {
                print_player(&player);

                if player_draw() {
                    player.draw(&mut deck);
                } else {
                    player.active = false;
                }
            }
        }

        if dealer.active {
            if dealer.sum() >= 17 {
                dealer.active = false;
            } else {
                dealer.draw(&mut deck);
            }
        }
    }

    print_player(&player);

    println!("Dealer's hand is:");
    dealer.show_hand();
    println!("=> {}\n", dealer.sum());

    check_win(&player, &dealer);
}

fn player_draw() -> bool {
    loop {
        println!("Draw another card? (yes/no)");

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

fn print_player(p: &Player) {
    println!("Player's hand is:");
    p.show_hand();
    println!("=> {}\n", p.sum());
}

fn check_win(player: &Player, dealer: &Player) {
    if player.sum() > 21 && dealer.sum() > 21 {
        println!("Everyone loses?");
    } else if player.sum() > 21 {
        println!("Dealer wins");
    } else if dealer.sum() > 21 {
        println!("Player wins");
    } else if dealer.sum() > player.sum() {
        println!("Dealer wins");
    } else if player.sum() > dealer.sum() {
        println!("Player wins");
    } else {
        println!("Equal score, now it's hard. Dealer probably won, but shit who knows.");
    }
}
