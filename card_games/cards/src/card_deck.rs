use rand::Rng;

use crate::card::{rank::Rank, suit::Suit, Card};

pub struct CardDeck {
    n: usize,
    deck: Vec<Card>,
}
impl CardDeck {
    pub const DECKSIZE: usize = 52;

    pub fn instantiate() -> CardDeck {
        let mut deck = CardDeck {
            n: CardDeck::DECKSIZE,
            deck: Vec::with_capacity(CardDeck::DECKSIZE),
        };
        for r in Rank::iterator() {
            for s in Suit::iterator() {
                deck.deck.push(Card { rank: r, suit: s });
            }
        }
        deck
    }

    pub fn shuffled() -> CardDeck {
        let mut deck = CardDeck::instantiate();
        deck.shuffle();
        deck
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn draw(&mut self) -> Option<Card> {
        if let Some(top) = self.deck.pop() {
            self.n -= 1;
            Some(top)
        } else {
            None
        }
    }

    pub fn shuffle(&mut self) {
        const N: usize = 1000;
        let i: usize = 0;
        let mut j: usize;

        for _ in 0..N {
            j = rand::thread_rng().gen_range(1..CardDeck::DECKSIZE);
            self.deck.swap(i, j);
        }
    }
}
