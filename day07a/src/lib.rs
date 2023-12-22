use combine::EasyParser;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::string::String;

use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use cards::card::Value;
use combine::{
    many1,
    parser::char::{alpha_num, digit, newline, space},
    sep_by, ParseError, Parser, Stream,
};

#[derive(Debug, Clone, Default, clap::Parser)]
pub struct Opts {
    #[clap(short, long)]
    pub input_filename: PathBuf,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> i32 {
        match *self {
            HandType::HighCard => 0,
            HandType::OnePair => 1,
            HandType::TwoPair => 2,
            HandType::ThreeOfAKind => 3,
            HandType::FullHouse => 4,
            HandType::FourOfAKind => 5,
            HandType::FiveOfAKind => 6,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.value().cmp(&other.hand_type.value());
        }
        for (c1, c2) in self.hand.chars().zip(other.hand.chars()) {
            if c1 != c2 {
                return convert(c1).cmp(&convert(c2));
            }
        }
        unreachable!();
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Debug, Eq, Hash)]
struct Hand {
    hand: String,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn new(hand: String, bid: usize) -> Self {
        let hand_type: HandType = find_hand_type(&hand);
        Self {
            hand,
            hand_type,
            bid,
        }
    }
}

fn convert(c: char) -> cards::card::Value {
    match c {
        'A' => Value::Ace,
        'K' => Value::King,
        'Q' => Value::Queen,
        'J' => Value::Jack,
        'T' => Value::Ten,
        '9' => Value::Nine,
        '8' => Value::Eight,
        '7' => Value::Seven,
        '6' => Value::Six,
        '5' => Value::Five,
        '4' => Value::Four,
        '3' => Value::Three,
        '2' => Value::Two,
        c => unreachable!("Wrong value: {}", c),
    }
}

fn find_hand_type(hand: &str) -> HandType {
    let mut counts: HashMap<Value, u8> = HashMap::new();
    hand.chars().for_each(|c| {
        let card = convert(c);
        *counts.entry(card).or_insert(0) += 1;
    });
    let mut counts = counts.into_values().sorted().collect::<Vec<u8>>();
    match counts.pop() {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => match counts.pop() {
            Some(2) => HandType::FullHouse,
            _ => HandType::ThreeOfAKind,
        },
        Some(2) => match counts.pop() {
            Some(2) => HandType::TwoPair,
            _ => HandType::OnePair,
        },
        Some(1) => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

fn hand<Input>() -> impl Parser<Input, Output = Hand>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(alpha_num()).skip(space()).and(_usize()))
        .map(|(hand, bid): (String, usize)| Hand::new(hand, bid))
}

fn game<Input>() -> impl Parser<Input, Output = Vec<Hand>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by(hand(), newline())).map(|hands: Vec<Hand>| hands)
}

pub fn compute(input: &str) -> usize {
    let mut total = 0;
    let mut cnt = 1;

    game()
        .easy_parse(input)
        .unwrap()
        .0
        .into_iter()
        .sorted()
        .for_each(|hand| {
            total += hand.bid * cnt;
            cnt += 1;
        });
    total
}

#[cfg(test)]
mod tests {
    use combine::EasyParser;

    use super::*;
    #[test]
    fn test_parse() {
        let input = "32T3K 765
T55J5 684";
        let (game, rest) = game().easy_parse(input).unwrap();
        assert_eq!(
            game,
            vec![
                Hand {
                    hand: String::from("32T3K"),
                    hand_type: HandType::OnePair,
                    bid: 765
                },
                Hand {
                    hand: String::from("T55J5"),
                    hand_type: HandType::ThreeOfAKind,
                    bid: 684
                }
            ]
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_compare_hands() {
        let one_pair1 = Hand::new(String::from("T9T35"), 1);
        let one_pair2 = Hand::new(String::from("2AA46"), 1);
        let full_house = Hand::new(String::from("6AAA6"), 1);
        let three_of_a_kind1 = Hand::new(String::from("T88K8"), 1);
        let three_of_a_kind2 = Hand::new(String::from("Q777K"), 1);

        assert!(one_pair1 > one_pair2);
        assert!(full_house > one_pair1);
        assert_eq!(three_of_a_kind1.hand_type, HandType::ThreeOfAKind);
        assert_eq!(three_of_a_kind2.hand_type, HandType::ThreeOfAKind);
        assert!(three_of_a_kind2 > three_of_a_kind1);
    }
}
