use std::cmp::Ordering;
use std::collections::BTreeMap;

mod card;
use card::{Card, CardError, CardRank};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard(Vec<CardRank>),
    OnePair(Vec<CardRank>),
    TwoPair(Vec<CardRank>),
    ThreeOfAKind(Vec<CardRank>),
    Straight(CardRank),
    Flush(Vec<CardRank>),
    FullHouse(Vec<CardRank>),
    FourOfAKind(Vec<CardRank>),
    StraightFlush(CardRank),
}

#[derive(Debug, Clone)]
struct Hand<'a> {
    repr: &'a str,
    cards: Vec<Card>,
    rank_count: BTreeMap<CardRank, usize>,
}

impl<'a> Hand<'a> {
    fn new(repr: &'a str) -> Result<Self, CardError> {
        let mut cards = repr
            .split(' ')
            .map(Card::new)
            .collect::<Result<Vec<_>, _>>()?;
        let rank_count = Self::rank_counter(cards.clone());

        cards.sort();
        Ok(Self {
            repr,
            cards,
            rank_count,
        })
    }

    fn poker_rank(&self) -> HandRank {
        use HandRank::*;
        match (
            self.rank_of_kind(2),
            self.rank_of_kind(3),
            self.rank_straight(),
            self.rank_flush(),
            self.rank_of_kind(4),
        ) {
            (_, _, Some(rank), Some(_), _) => StraightFlush(rank),
            (_, _, _, _, Some(rank)) => FourOfAKind(rank),
            (Some(pair), Some(mut rank), _, _, _) => {
                rank.extend(pair);
                FullHouse(rank)
            }
            (_, _, _, Some(rank), _) => Flush(rank),
            (_, _, Some(rank), _, _) => Straight(rank),
            (_, Some(rank), _, _, _) => ThreeOfAKind(rank),
            (Some(rank), _, _, _, _) if rank.len() == 2 => TwoPair(rank),
            (Some(rank), _, _, _, _) if rank.len() == 1 => OnePair(rank),
            (_, _, _, _, _) => self.rank_cards(),
        }
    }

    fn rank_cards(&self) -> HandRank {
        HandRank::HighCard(
            self.cards
                .iter()
                .rev()
                .map(|card| card.rank)
                .collect::<Vec<_>>(),
        )
    }

    fn rank_of_kind(&self, n: usize) -> Option<Vec<CardRank>> {
        let ranks = self
            .rank_count
            .iter()
            .filter(|(_, count)| **count == n)
            .map(|(rank, _)| *rank)
            .rev()
            .collect::<Vec<_>>();
        if !ranks.is_empty() {
            Some(ranks)
        } else {
            None
        }
    }

    fn rank_straight(&self) -> Option<CardRank> {
        use CardRank::*;
        let ranks = self
            .cards
            .iter()
            .rev()
            .map(|card| card.rank)
            .collect::<Vec<_>>();

        let consecutive_cards = vec![
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ];

        if consecutive_cards
            .windows(5)
            .map(|x| x.iter().copied().collect::<Vec<_>>())
            .filter(|solns| *solns == ranks)
            .count()
            == 1
        {
            Some(ranks[0])
        } else if ranks == vec![Ace, Five, Four, Three, Two] {
            Some(Five)
        } else {
            None
        }
    }

    fn rank_flush(&self) -> Option<Vec<CardRank>> {
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

        Some(ranks)
    }

    fn rank_counter(cards: Vec<Card>) -> BTreeMap<CardRank, usize> {
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
            .find(|&ord| ord != Ordering::Equal)
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

impl Eq for Hand<'_> {}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.poker_rank()
                .cmp(&other.poker_rank())
                .then_with(|| self.cmp_rank(other)),
        )
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
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
        .iter()
        .map(|cards| Hand::new(cards).ok())
        .collect::<Option<Vec<Hand>>>()?;

    hands.sort();
    let hands_clone = hands.clone();

    let max = hands_clone.iter().last()?;
    Some(
        hands
            .into_iter()
            .filter(|hand| hand.cmp(max) == Ordering::Equal)
            .map(|hand| hand.repr)
            .collect(),
    )
}
