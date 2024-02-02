use std::collections::HashMap;
use std::string::String;

use combine::{
    attempt, choice, many1,
    parser::char::{char, digit, letter},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};

fn hash(v: &[u8]) -> usize {
    let mut current: usize = 0;
    for c in v.iter() {
        current += *c as usize;
        current *= 17;
        current %= 256;
    }
    current
}

#[derive(Debug, Default)]
struct Lenses(HashMap<usize, LensBox>);

#[derive(Debug, Default)]
struct LensBox {
    pub queue: Vec<(String, usize)>,
}

enum Actions {
    Remove(String),
    Add(String, u32),
}

fn manual<Input>() -> impl Parser<Input, Output = Lenses>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by1(
        choice!(
            attempt(many1(letter()).skip(char('-'))).map(|label: String| Actions::Remove(label)),
            (many1(letter()).skip(char('=')), digit()).map(
                |(label, focal_length): (String, char)| Actions::Add(
                    label,
                    focal_length.to_digit(10).unwrap()
                )
            )
        ),
        char(','),
    )
    .map(|steps: Vec<Actions>| {
        let mut lenses: Lenses = Lenses::default();
        for action in steps.into_iter() {
            match action {
                Actions::Remove(label) => {
                    let box_num = hash(label.as_bytes());
                    if let Some(lens_box) = lenses.0.get_mut(&box_num) {
                        lens_box
                            .queue
                            .retain(|(l, _): &(String, usize)| l != &label)
                    }
                }
                Actions::Add(label, focal_length) => {
                    let box_num = hash(label.as_bytes());
                    match lenses.0.get_mut(&box_num) {
                        Some(lens_box) => {
                            match lens_box
                                .queue
                                .iter_mut()
                                .find(|(l, _): &&mut (String, usize)| l == &label)
                            {
                                Some((_, ref mut f)) => {
                                    *f = focal_length as usize;
                                }
                                None => lens_box.queue.push((label, focal_length as usize)),
                            }
                        }
                        None => {
                            lenses.0.insert(
                                box_num,
                                LensBox {
                                    queue: vec![(label, focal_length as usize)],
                                },
                            );
                        }
                    }
                }
            }
        }
        lenses
    })
}

pub fn compute(input: &str) -> usize {
    let lenses = manual().easy_parse(input).unwrap().0;
    let mut total = 0;
    lenses
        .0
        .into_iter()
        .for_each(|(num_box, lens_box): (usize, LensBox)| {
            lens_box.queue.iter().enumerate().for_each(
                |(i, (_, focal_lenght)): (usize, &(String, usize))| {
                    total += (num_box + 1) * (i + 1) * focal_lenght;
                },
            )
        });
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let (output, rest) = manual().easy_parse(input).unwrap();
        println!("{:?}", output);
        assert!(rest.is_empty());
        assert_eq!(
            output.0.get(&3).unwrap().queue,
            vec![
                (String::from("ot"), 7),
                (String::from("ab"), 5),
                (String::from("pc"), 6)
            ]
        );
    }
}
