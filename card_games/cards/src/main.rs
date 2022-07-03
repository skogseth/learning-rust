use cards::{CardDeck, Card, card::Rank, card::Suit};

fn main() {
    println!("Tests for cards crate: \n");

    test_card();

    for s in Suit::iterator() { println!("{}", s); }

    for r in Rank::iterator() { println!("{}", r); }

    test_deck();

    test_shuffle();
}

fn test_card() {
    let s: Suit = Suit::Diamonds;
    println!("Suit: {}", s);

    let r: Rank = Rank::Jack;
    println!("Rank: {}", r);

    let card1 = Card { rank: Rank::Ace, suit: Suit::Spades };
    println!("{}", card1);
}

fn test_deck() {
    let mut deck = CardDeck::instantiate();
    println!("Size of deck: {}", deck.size());
    for _ in 0..(CardDeck::DECKSIZE+4) {
        if let Some(card) = deck.draw() {
            println!("Card: {}", card);
        } else {
            println!("Empty deck");
        }
    }
}

fn test_shuffle() {
    let mut deck = CardDeck::instantiate();
    deck.shuffle();
    for _ in 0..4 {
        if let Some(card) = deck.draw() {
            println!("Card: {}", card);
        } else {
            println!("Empty deck");
        }
    }
}
