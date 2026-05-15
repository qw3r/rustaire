use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        }
    }

    pub fn is_red(&self) -> bool {
        matches!(self, Suit::Hearts | Suit::Diamonds)
    }

    pub fn all() -> [Suit; 4] {
        [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Ace,
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

impl Rank {
    pub fn value(&self) -> u8 {
        match self {
            Rank::Ace => 1,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }

    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub face_up: bool,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit,
            rank,
            face_up: false,
        }
    }

    pub fn is_red(&self) -> bool {
        self.suit.is_red()
    }

    pub fn can_stack_on_tableau(&self, other: &Card) -> bool {
        self.is_red() != other.is_red() && self.rank.value() + 1 == other.rank.value()
    }

    pub fn can_stack_on_foundation(&self, top: Option<&Card>, suit: Suit) -> bool {
        if self.suit != suit {
            return false;
        }
        match top {
            None => self.rank == Rank::Ace,
            Some(top_card) => self.rank.value() == top_card.rank.value() + 1,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.face_up {
            write!(f, "{}{}", self.rank.symbol(), self.suit.symbol())
        } else {
            write!(f, "??")
        }
    }
}
