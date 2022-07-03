use cards::{CardDeck, Card, card::Rank};

pub struct Player {
    cards: Vec<Card>,
    sum: u16,
    pub active: bool,
}
impl Player {
    pub fn initialize() -> Player {
        Player {
            cards: Vec::with_capacity(2),
            sum: 0,
            active: true,
        }
    }

    pub fn draw(&mut self, deck: &mut CardDeck) {
        let card = draw_card(deck);

        self.cards.push(card);

        self.sum = 0;
        let mut aces: u16 = 0;

        for card in &self.cards {
            if card.rank == Rank::Ace {
                aces += 1;
            } else if card.rank == Rank::Jack || card.rank == Rank::Queen || card.rank == Rank::King {
                self.sum += 10 as u16;
            } else {
                self.sum += card.rank as u16;
            }
        }
        while aces > 0 {
            if self.sum + aces * 11 <= 21 {
                self.sum += aces * 11;
                break;
            } else {
                self.sum += 1;
                aces -= 1;
            }
        }
    }

    pub fn sum(&self) -> u16 { self.sum }

    pub fn show_hand(&self) {
        for i in 0..self.cards.len() {
            println!("{}", self.cards[i]);
        }
    }

    pub fn show_first_card(&self) {
        if self.cards.len() > 0 {
            println!("{}", self.cards[0]);
        }
    }
}

fn draw_card(deck: &mut CardDeck) -> Card {
    if let Some(card) = deck.draw() {
        card
    } else {
        *deck = CardDeck::shuffled();
        draw_card(deck)
    }
}
