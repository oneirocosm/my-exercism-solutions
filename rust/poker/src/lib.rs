use std::cmp::Ordering;
use std::collections::BTreeMap;

mod card;
use card::{Card, CardError, Rank};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum PokerRank {
    HighCard(Vec<Rank>),
    OnePair(Vec<Rank>),
    TwoPair(Vec<Rank>),
    ThreeOfAKind(Vec<Rank>),
    LowAceStraight(Vec<Rank>),
    Straight(Vec<Rank>),
    Flush(Vec<Rank>),
    FullHouse(Vec<Rank>),
    FourOfAKind(Vec<Rank>),
    StraightFlush(Vec<Rank>),
}

#[derive(Debug, Clone)]
struct Hand<'a> {
    repr: &'a str,
    cards: Vec<Card>,
    rank_count: BTreeMap<Rank, usize>,
}

impl<'a> Hand<'a> {
    fn new(repr: &'a str) -> Result<Self, CardError> {
        let mut cards = repr
            .split(" ")
            .map(Card::new)
            .collect::<Result<Vec<_>, _>>()?;
        let rank_count = Self::rank_counter(cards.clone());

        cards.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        Ok(Self {
            repr,
            cards,
            rank_count,
        })
    }

    fn poker_rank(&self) -> PokerRank {
        use PokerRank::*;
        match (
            self.rank_2_of_kind(),
            self.rank_3_of_kind(),
            self.rank_straight(),
            self.rank_flush(),
            self.rank_full_house(),
            self.rank_4_of_kind(),
        ) {
            (_, _, Some(Straight(r)), Some(Flush(_)), _, _) => StraightFlush(r),
            (_, _, _, _, _, Some(rank)) => rank,
            (_, _, _, _, Some(rank), _) => rank,
            (_, _, _, Some(rank), _, _) => rank,
            (_, _, Some(rank), _, _, _) => rank,
            (_, Some(rank), _, _, _, _) => rank,
            (Some(rank), _, _, _, _, _) => rank,
            (_, _, _, _, _, _) => self.rank_cards(),
        }
    }

    fn rank_cards(&self) -> PokerRank {
        PokerRank::HighCard(
            self.cards
                .iter()
                .rev()
                .map(|card| card.rank)
                .collect::<Vec<_>>(),
        )
    }

    fn rank_2_of_kind(&self) -> Option<PokerRank> {
        use PokerRank::{OnePair, TwoPair};

        match self
            .rank_count
            .iter()
            .filter(|(_, count)| **count == 2)
            .map(|(rank, _)| *rank)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        {
            ranks if ranks.len() == 2 => Some(TwoPair(ranks)),
            ranks if ranks.len() == 1 => Some(OnePair(ranks)),
            _ => None,
        }
    }

    fn rank_3_of_kind(&self) -> Option<PokerRank> {
        use PokerRank::ThreeOfAKind;

        match self
            .rank_count
            .iter()
            .filter(|(_, count)| **count == 3)
            .map(|(rank, _)| *rank)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        {
            ranks if ranks.len() == 1 => Some(ThreeOfAKind(ranks)),
            _ => None,
        }
    }

    fn rank_4_of_kind(&self) -> Option<PokerRank> {
        use PokerRank::FourOfAKind;

        match self
            .rank_count
            .iter()
            .filter(|(_, count)| **count == 4)
            .map(|(rank, _)| *rank)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
        {
            ranks if ranks.len() == 1 => Some(FourOfAKind(ranks)),
            _ => None,
        }
    }

    fn rank_full_house(&self) -> Option<PokerRank> {
        use PokerRank::{FullHouse, OnePair, ThreeOfAKind};
        let mut ranks = match self.rank_3_of_kind() {
            Some(ThreeOfAKind(r)) => Some(r),
            _ => None,
        }?;

        let pair = match self.rank_2_of_kind() {
            Some(OnePair(r)) => Some(r),
            _ => None,
        }?;

        ranks.extend(pair);
        Some(FullHouse(ranks))
    }

    fn rank_straight(&self) -> Option<PokerRank> {
        use Rank::*;
        let ranks = self.cards.iter().map(|card| card.rank).collect::<Vec<_>>();

        if vec![
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ]
        .windows(5)
        .map(|x| x.into_iter().cloned().collect::<Vec<_>>())
        .filter(|solns| *solns == ranks)
        .rev()
        .count()
            == 1
        {
            Some(PokerRank::Straight(ranks))
        } else if ranks == vec![Two, Three, Four, Five, Ace] {
            Some(PokerRank::LowAceStraight(ranks))
        } else {
            None
        }
    }

    fn rank_flush(&self) -> Option<PokerRank> {
        let (ranks, _) = self
            .cards
            .iter()
            .rev()
            .try_fold((Vec::new(), None), |(mut ranks, suit), card| {
                if let Some(s) = suit {
                    if card.suit == s {
                        ranks.push(card.rank);
                        Ok((ranks, suit))
                    } else {
                        Err("Ranks do not match")
                    }
                } else {
                    ranks.push(card.rank);
                    Ok((ranks, Some(card.suit)))
                }
            })
            .ok()?;

        Some(PokerRank::Flush(ranks))
    }

    fn rank_counter(cards: Vec<Card>) -> BTreeMap<Rank, usize> {
        cards.into_iter().fold(BTreeMap::new(), |mut count, card| {
            *count.entry(card.rank).or_insert(0) += 1;
            count
        })
    }

    fn cmp_rank(&self, other: &Self) -> Ordering {
        match self
            .cards
            .iter()
            .zip(other.cards.iter())
            .map(|(s, o)| s.rank.cmp(&o.rank))
            .rev()
            .skip_while(|&ord| ord == Ordering::Equal)
            .next()
        {
            None => Ordering::Equal,
            Some(ord) => ord,
        }
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.poker_rank() == other.poker_rank()
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.poker_rank().cmp(&other.poker_rank()) {
            Ordering::Equal => self.cmp_rank(other),
            ord => ord,
        })
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() {
        return None;
    }

    let mut hands = hands
        .into_iter()
        .map(|cards| Hand::new(cards).ok())
        .collect::<Option<Vec<Hand>>>()?;

    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let hands_clone = hands.clone();

    let max = hands_clone.iter().last().unwrap();
    Some(
        hands
            .into_iter()
            .filter(|hand| hand.partial_cmp(max).unwrap_or(Ordering::Equal) == Ordering::Equal)
            .map(|hand| hand.repr)
            .collect(),
    )
}
