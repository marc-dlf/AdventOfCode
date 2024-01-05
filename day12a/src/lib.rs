use std::string::String;

//use ::polyfit_rs::polyfit_rs::polyfit;
use combine::{
    many1,
    parser::char::{char, digit, newline, space},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

fn hot_springs<Input>() -> impl Parser<Input, Output = Vec<HotSpring>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by1(
        (
            many1::<String, _, _>(char('?').or(char('#').or(char('.')))).skip(space()),
            sep_by1::<Vec<usize>, _, _, _>(_usize(), char(',')),
        ),
        newline(),
    ),)
        .map(|(springs,): (Vec<(String, Vec<usize>)>,)| {
            springs
                .into_iter()
                .map(|(springs, damaged)| HotSpring { springs, damaged })
                .collect::<Vec<HotSpring>>()
        })
}

#[derive(Debug)]
pub struct HotSpring {
    springs: String,
    damaged: Vec<usize>,
}

pub fn decode(spring: HotSpring) -> usize {
    let remaining_damaged = spring.damaged.repeat(3);
    let springs = spring.springs.repeat(3);
    backtrack(springs.chars().next(), springs, remaining_damaged, false)
}

pub fn backtrack(
    c: Option<char>,
    mut remaining_springs: String,
    mut remaining_damaged: Vec<usize>,
    contiguous: bool,
) -> usize {
    if let Some(c) = c {
        match c {
            '?' => {
                let mut total = 0;
                total += backtrack(
                    Some('.'),
                    remaining_springs.clone(),
                    remaining_damaged.clone(),
                    contiguous,
                );
                total += backtrack(Some('#'), remaining_springs, remaining_damaged, contiguous);
                total
            }
            '.' => {
                match remaining_damaged.first() {
                    Some(&0) => {
                        remaining_damaged.remove(0);
                    }
                    Some(_) => {
                        if contiguous {
                            return 0;
                        }
                    }
                    _ => {}
                };
                remaining_springs.remove(0);
                backtrack(
                    remaining_springs.chars().next(),
                    remaining_springs,
                    remaining_damaged,
                    false,
                )
            }

            '#' => match remaining_damaged.first() {
                Some(&0) => 0,
                Some(_) => {
                    *remaining_damaged.get_mut(0).unwrap() -= 1;
                    remaining_springs.remove(0);
                    backtrack(
                        remaining_springs.chars().next(),
                        remaining_springs,
                        remaining_damaged,
                        true,
                    )
                }
                None => 0,
            },
            c => unreachable!("Impossible char : {}", c),
        }
    } else if (remaining_damaged.is_empty())
        | ((remaining_damaged.first() == Some(&0)) & (remaining_damaged.len() == 1))
    {
        1
    } else {
        0
    }
}

pub fn compute(input: &str) -> usize {
    let (hot_springs, _) = hot_springs().easy_parse(input).unwrap();
    let mut total = 0;
    hot_springs.into_iter().for_each(|s| total += decode(s));
    total
    //Part 1
    //image.compute_shortest_paths(2)
    //Part 2
    //image.compute_shortest_paths(1000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let (output, rest) = hot_springs().easy_parse(input).unwrap();
        assert!(rest.is_empty());
        let first_hotspring = output.first().unwrap();
        assert_eq!(first_hotspring.springs, String::from("???.###"));
        assert_eq!(first_hotspring.damaged, vec![1, 1, 3]);
    }

    #[test]
    fn test_chars() {
        let input: &str = "abc";
        let mut input = input.chars();
        assert_eq!(input.next(), Some('a'));
        assert_eq!(input.next(), Some('b'));
    }

    #[test]
    fn test_decode() {
        let hs1: HotSpring = HotSpring {
            springs: String::from("???.###"),
            damaged: vec![1, 1, 3],
        };
        let hs2: HotSpring = HotSpring {
            springs: String::from("?###????????"),
            damaged: vec![3, 2, 1],
        };
        let hs3: HotSpring = HotSpring {
            springs: String::from(".????.######..#####."),
            damaged: vec![1, 6, 5],
        };
        let hs4: HotSpring = HotSpring {
            springs: String::from("??.?"),
            damaged: vec![2],
        };

        assert_eq!(decode(hs1), 1);
        assert_eq!(decode(hs2), 10);
        assert_eq!(decode(hs3), 4);
        assert_eq!(decode(hs4), 1);
    }
}
