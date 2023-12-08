use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardWithJoker {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl CardWithJoker {
    fn to_card(&self) -> Card {
        match self {
            CardWithJoker::A => Card::A,
            CardWithJoker::K => Card::K,
            CardWithJoker::Q => Card::Q,
            CardWithJoker::T => Card::T,
            CardWithJoker::_9 => Card::_9,
            CardWithJoker::_8 => Card::_8,
            CardWithJoker::_7 => Card::_7,
            CardWithJoker::_6 => Card::_6,
            CardWithJoker::_5 => Card::_5,
            CardWithJoker::_4 => Card::_4,
            CardWithJoker::_3 => Card::_3,
            CardWithJoker::_2 => Card::_2,
            CardWithJoker::J => Card::J,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct HandWithJoker {
    cards: Vec<CardWithJoker>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn kind(&self) -> Kind {
        let counts: Vec<usize> = self
            .cards
            .iter()
            .sorted_unstable()
            .dedup_with_count()
            .map(|(c, _)| c)
            .sorted_unstable()
            .collect();

        if counts == vec![5] {
            Kind::FiveOfAKind
        } else if counts == vec![1, 4] {
            Kind::FourOfAKind
        } else if counts == vec![2, 3] {
            Kind::FullHouse
        } else if counts == vec![1, 1, 3] {
            Kind::ThreeOfAKind
        } else if counts == vec![1, 2, 2] {
            Kind::TwoPair
        } else if counts == vec![1, 1, 1, 2] {
            Kind::OnePair
        } else {
            Kind::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind_ord = self.kind().cmp(&other.kind());
        if kind_ord == std::cmp::Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            kind_ord
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HandWithJoker {
    fn best_hand(&self) -> Hand {
        let mut hands = vec![];
        for c1 in self.to_cards(&self.cards[0]) {
            for c2 in self.to_cards(&self.cards[1]) {
                for c3 in self.to_cards(&self.cards[2]) {
                    for c4 in self.to_cards(&self.cards[3]) {
                        for c5 in self.to_cards(&self.cards[4]) {
                            hands.push(Hand {
                                cards: vec![
                                    c1.clone(),
                                    c2.clone(),
                                    c3.clone(),
                                    c4.clone(),
                                    c5.clone(),
                                ],
                            });
                        }
                    }
                }
            }
        }

        hands.into_iter().max().unwrap()
    }

    fn to_cards(&self, c: &CardWithJoker) -> Vec<Card> {
        if c == &CardWithJoker::J {
            vec![
                Card::A,
                Card::K,
                Card::Q,
                Card::J,
                Card::T,
                Card::_9,
                Card::_8,
                Card::_7,
                Card::_6,
                Card::_5,
                Card::_4,
                Card::_3,
                Card::_2,
            ]
        } else {
            vec![c.to_card()]
        }
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let kind_ord = self.best_hand().kind().cmp(&other.best_hand().kind());
        if kind_ord == std::cmp::Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            kind_ord
        }
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                '9' => Ok(Card::_9),
                '8' => Ok(Card::_8),
                '7' => Ok(Card::_7),
                '6' => Ok(Card::_6),
                '5' => Ok(Card::_5),
                '4' => Ok(Card::_4),
                '3' => Ok(Card::_3),
                '2' => Ok(Card::_2),
                _ => Err(anyhow!("invalid card: {}", c)),
            })
            .collect::<Result<Vec<Card>>>()?;
        Ok(Hand { cards })
    }
}

impl std::str::FromStr for HandWithJoker {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| match c {
                'A' => Ok(CardWithJoker::A),
                'K' => Ok(CardWithJoker::K),
                'Q' => Ok(CardWithJoker::Q),
                'J' => Ok(CardWithJoker::J),
                'T' => Ok(CardWithJoker::T),
                '9' => Ok(CardWithJoker::_9),
                '8' => Ok(CardWithJoker::_8),
                '7' => Ok(CardWithJoker::_7),
                '6' => Ok(CardWithJoker::_6),
                '5' => Ok(CardWithJoker::_5),
                '4' => Ok(CardWithJoker::_4),
                '3' => Ok(CardWithJoker::_3),
                '2' => Ok(CardWithJoker::_2),
                _ => Err(anyhow!("invalid card: {}", c)),
            })
            .collect::<Result<Vec<CardWithJoker>>>()?;
        Ok(HandWithJoker { cards })
    }
}

fn main() -> Result<()> {
    let input = read_input("input.txt")?;
    let mut hands: Vec<(Hand, u32)> = input
        .iter()
        .map(|l| {
            let (hand_str, bid_str) = l.split_once(" ").unwrap();
            Ok((hand_str.parse::<Hand>()?, bid_str.parse::<u32>()?))
        })
        .collect::<Result<Vec<_>>>()?;
    hands.sort_unstable();

    let total_winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) as u32 * b)
        .sum();
    dbg!(total_winnings);

    let mut hands_with_joker: Vec<(HandWithJoker, u32)> = input
        .iter()
        .map(|l| {
            let (hand_str, bid_str) = l.split_once(" ").unwrap();
            Ok((hand_str.parse::<HandWithJoker>()?, bid_str.parse::<u32>()?))
        })
        .collect::<Result<Vec<_>>>()?;
    hands_with_joker.sort_unstable();

    let total_winnings_2: u32 = hands_with_joker
        .iter()
        .enumerate()
        .map(|(i, (_, b))| (i + 1) as u32 * b)
        .sum();
    dbg!(total_winnings_2);

    Ok(())
}

fn read_input<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().map(|l| Ok(l?)).collect()
}

#[cfg(test)]
mod tests {
    use super::{Card, CardWithJoker, Hand, HandWithJoker};
    use anyhow::Result;

    #[test]
    fn test_best_hand() -> Result<()> {
        assert_eq!(
            HandWithJoker {
                cards: vec![
                    CardWithJoker::A,
                    CardWithJoker::A,
                    CardWithJoker::A,
                    CardWithJoker::A,
                    CardWithJoker::J
                ]
            }
            .best_hand(),
            Hand {
                cards: vec![Card::A, Card::A, Card::A, Card::A, Card::A,]
            }
        );
        Ok(())
    }
}
