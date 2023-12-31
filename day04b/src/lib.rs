use combine::EasyParser;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use combine::{
    many1,
    parser::char::{char, digit, spaces, string},
    parser::choice::choice,
    sep_by, sep_end_by1, ParseError, Parser, Stream,
};

#[derive(Debug, Clone, Default, clap::Parser)]
pub struct Opts {
    #[clap(short, long)]
    pub input_filename: PathBuf,
}

struct ScratchCard {
    winning_num: NumList,
    owned_num: NumList,
    num_copy: usize,
}

impl ScratchCard {
    fn count_match(&self) -> usize {
        self.winning_num
            .0
            .intersection(&self.owned_num.0)
            .collect::<Vec<&usize>>()
            .len()
    }

    fn new(input: &str) -> Self {
        scratch_card().easy_parse(input).unwrap().0
    }
}

pub fn compute(filename: PathBuf) {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    let mut out = 0;
    let mut scratch_cards: Vec<ScratchCard> = vec![];
    for l in reader.lines() {
        //scratch_cards.insert(i, ScratchCard::new(&l.unwrap()));
        scratch_cards.push(ScratchCard::new(&l.unwrap()))
    }
    let n = scratch_cards.len();
    for i in 0..n {
        for j in 1..scratch_cards[i].count_match() + 1 {
            let k = i + j;
            if k < n {
                scratch_cards[k].num_copy += scratch_cards[i].num_copy;
            }
        }
        out += scratch_cards[i].num_copy;
    }
    println!("Result:{:?}", out);
}

#[derive(Clone)]
struct NumList(HashSet<usize>);

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

fn numlist<Input>() -> impl Parser<Input, Output = NumList>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((sep_end_by1(_usize(), spaces()), sep_by(_usize(), spaces())))
        .map(|numbers: Vec<usize>| numbers)
        .map(|numbers| NumList(HashSet::from_iter(numbers)))
}

fn scratch_card<Input>() -> impl Parser<Input, Output = ScratchCard>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        string("Card").skip(spaces()),
        _usize(),
        char(':').skip(spaces()),
        numlist(),
        char('|').skip(spaces()),
        numlist(),
    )
        .map(
            |(_, _, _, winning_num, _, owned_num): (_, _, _, NumList, _, NumList)| ScratchCard {
                winning_num,
                owned_num,
                num_copy: 1,
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse_numlist() {
        let input = "1 21 53 59";
        let (num_list, _): (NumList, &str) = numlist().easy_parse(input).unwrap();
        assert_eq!(num_list.0, HashSet::from_iter(vec![1, 21, 53, 59]));
    }

    // #[test]
    // fn test_parse_scratchcard() {
    //     let input = "Card 123: 1 21 53 59 | 42 45 66";
    //     let (
    //         ScratchCard {
    //             winning_num,
    //             owned_num,
    //         },
    //         _,
    //     ): (ScratchCard, &str) = scratch_card().easy_parse(input).unwrap();
    //     assert_eq!(winning_num.0, HashSet::from_iter(vec![1, 21, 53, 59]));
    //     assert_eq!(owned_num.0, HashSet::from_iter(vec![42, 45, 66]));
    // }
}
