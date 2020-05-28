use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardRank {
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
    Ace,
}

impl From<u8> for CardRank {
    fn from(val: u8) -> Self {
        match val {
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            _ => Self::Ace,
        }
    }
}

impl Into<u8> for CardRank {
    fn into(self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

#[derive(Debug, Copy, Clone, Hash)]
pub enum CardError<'a> {
    InvalidRank(&'a str),
    InvalidSuit(&'a str),
}

#[derive(Debug, Copy, Clone, Hash, PartialEq)]
pub struct Card {
    pub rank: CardRank,
    pub suit: Suit,
}

impl Card {
    pub fn new(name: &str) -> Result<Self, CardError> {
        let len = name.len();
        let (rank, suit) = name.split_at(len - 1);

        let rank = match rank {
            "2" => Ok(CardRank::Two),
            "3" => Ok(CardRank::Three),
            "4" => Ok(CardRank::Four),
            "5" => Ok(CardRank::Five),
            "6" => Ok(CardRank::Six),
            "7" => Ok(CardRank::Seven),
            "8" => Ok(CardRank::Eight),
            "9" => Ok(CardRank::Nine),
            "10" => Ok(CardRank::Ten),
            "J" => Ok(CardRank::Jack),
            "Q" => Ok(CardRank::Queen),
            "K" => Ok(CardRank::King),
            "A" => Ok(CardRank::Ace),
            r => Err(CardError::InvalidRank(r)),
        }?;

        let suit = match suit {
            "S" => Ok(Suit::Spades),
            "C" => Ok(Suit::Clubs),
            "H" => Ok(Suit::Hearts),
            "D" => Ok(Suit::Diamonds),
            s => Err(CardError::InvalidSuit(s)),
        }?;

        Ok(Self { rank, suit })
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

//impl Ord for Card {
//    fn cmp(&self, other: &Self) -> Ordering {
//        match self.rank.cmp(&other.rank) {
//            Ordering::Equal => self.suit.cmp(&other.suit),
//            ord => ord,
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ace_of_spades() {
        let card = Card::new("AS").unwrap();
        assert_eq!(card.rank, CardRank::Ace);
        assert_eq!(card.suit, Suit::Spades);
    }

    #[test]
    fn create_10_of_clubs() {
        let card = Card::new("10C").unwrap();
        assert_eq!(card.rank, CardRank::Ten);
        assert_eq!(card.suit, Suit::Clubs);
    }

    #[test]
    fn invalid_rank_error() {
        let card = Card::new("11S");
        assert!(card.is_err());
    }

    #[test]
    fn invalid_suit_error() {
        let card = Card::new("8Z");
        assert!(card.is_err());
    }

    #[test]
    fn sorts_by_rank() {
        let bigger = Card::new("3C").unwrap();
        let smaller = Card::new("2C").unwrap();
        assert!(bigger > smaller);
    }

    #[test]
    fn sort_ace_above_king() {
        let bigger = Card::new("AH").unwrap();
        let smaller = Card::new("KH").unwrap();
        assert!(bigger > smaller);
    }
}
