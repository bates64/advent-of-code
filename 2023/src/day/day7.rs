use std::{cmp::Ordering, collections::HashMap};

use super::Error;

const INPUT: &str = include_str!("day7.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Card {
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

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card: {}", value),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Strength {
    fn calc(cards: &[Card]) -> Strength {
        /*
        Five of a kind, where all five cards have the same label: AAAAA
        Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
        Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
        Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        High card, where all cards' labels are distinct: 23456 */
        let mut counts: HashMap<Card, u32> = HashMap::new();
        for &card in cards {
            *counts.entry(card).or_default() += 1;
        }
        let mut counts: Vec<u32> = counts.into_values().collect();
        counts.sort();

        assert_eq!(counts.iter().sum::<u32>(), 5);

        match &counts[..] {
            &[5] => Strength::FiveOfAKind,
            &[1, 4] => Strength::FourOfAKind,
            &[2, 3] => Strength::FullHouse,
            &[1, 1, 3] => Strength::ThreeOfAKind,
            &[1, 2, 2] => Strength::TwoPair,
            &[1, 1, 1, 2] => Strength::OnePair,
            &[1, 1, 1, 1, 1] => Strength::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct Hand {
    strength: Strength,
    cards: Vec<Card>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.strength.cmp(&other.strength) {
            Ordering::Equal => {
                // best first card wins
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }
                Ordering::Equal
            }
            o => o,
        }
    }
}

pub fn part1() -> Result<String, Error> {
    let mut hands: Vec<Hand> = INPUT
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let cards: Vec<Card> = split.next().unwrap().chars().map(Into::into).collect();
            let bid: usize = split.next().unwrap().parse().unwrap();
            Hand {
                strength: Strength::calc(&cards),
                cards,
                bid,
            }
        })
        .collect();
    hands.sort();

    let mut total_winnings = 0;
    for (rank, hand) in hands.into_iter().enumerate() {
        total_winnings += (rank + 1) * hand.bid;
    }
    Ok(total_winnings.to_string())
}

pub fn part2() -> Result<String, Error> {
    Err(Error::Unimplemented)
}
