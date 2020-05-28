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

    fn poker_rank(&self) -> HandRank {
        use HandRank::*;
        match (
            self.rank_2_of_kind(),
            self.rank_3_of_kind(),
            self.rank_straight(),
            self.rank_flush(),
            self.rank_4_of_kind(),
        ) {
            (_, _, Some(Straight(r)), Some(Flush(_)), _) => StraightFlush(r),
            (_, _, _, _, Some(rank)) => rank,
            (Some(OnePair(pair)), Some(ThreeOfAKind(mut rank)), _, _, _) => {
                rank.extend(pair);
                FullHouse(rank)
            }
            (_, _, _, Some(rank), _) => rank,
            (_, _, Some(rank), _, _) => rank,
            (_, Some(rank), _, _, _) => rank,
            (Some(rank), _, _, _, _) => rank,
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

    fn rank_2_of_kind(&self) -> Option<HandRank> {
        use HandRank::{OnePair, TwoPair};

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

    fn rank_3_of_kind(&self) -> Option<HandRank> {
        use HandRank::ThreeOfAKind;

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

    fn rank_4_of_kind(&self) -> Option<HandRank> {
        use HandRank::FourOfAKind;

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

    fn rank_straight(&self) -> Option<HandRank> {
        use CardRank::*;
        let ranks = self
            .cards
            .iter()
            .rev()
            .map(|card| card.rank)
            .collect::<Vec<_>>();

        if vec![
            Ace, King, Queen, Jack, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two,
        ]
        .windows(5)
        .map(|x| x.into_iter().cloned().collect::<Vec<_>>())
        .filter(|solns| *solns == ranks)
        .rev()
        .count()
            == 1
        {
            Some(HandRank::Straight(ranks[0]))
        } else if ranks == vec![Ace, Five, Four, Three, Two] {
            Some(HandRank::Straight(Five))
        } else {
            None
        }
    }

    fn rank_flush(&self) -> Option<HandRank> {
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

        Some(HandRank::Flush(ranks))
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
