#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

use self::Suit::*;

impl Suit {
    fn to_str(&self) -> &str {
        match self {
            Clubs => "Clubs",
            Diamonds => "Diamonds",
            Hearts => "Hearts",
            Spades => "Spades",
        }
    }

    pub fn iterator() -> impl Iterator<Item = Suit> {
        [Clubs, Diamonds, Hearts, Spades].iter().copied()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}



#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

use self::Rank::*;

impl Rank {
    fn to_str(&self) -> &str {
        match self {
            Ace => "Ace",
            Two => "Two",
            Three => "Three",
            Four => "Four",
            Five => "Five",
            Six => "Six",
            Seven => "Seven",
            Eight => "Eight",
            Nine => "Nine",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
        }
    }

    pub fn iterator() -> impl Iterator<Item = Rank> {
        [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King].iter().copied()
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
