use rand::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Suit {
    Heart,
    Diamonds,
    Spade,
    Club
}
#[derive(PartialEq, Eq, Debug)]
pub enum Rank {
    Ace,
    Jack,
    Queen,
    King,
    Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        Self::translate(thread_rng().gen_range(1..=4))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Self::Heart,
            2 => Self::Diamonds,
            3 => Self::Spade,
            4 => Self::Club,
            _ => Self::random()
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        Self::translate(thread_rng().gen_range(1..=13))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Self::Ace,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => Self::Number(value)
        }
    }
}
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    if card.suit == Suit::Spade && card.rank == Rank::Ace {
        return true
    }
    false
}