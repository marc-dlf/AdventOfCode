use combine::EasyParser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use combine::{
    many1,
    parser::char::{char, digit, spaces, string},
    parser::repeat::skip_until,
    sep_by, ParseError, Parser, Stream,
};

#[derive(Debug, Clone, Default, clap::Parser)]
pub struct Opts {
    #[clap(short, long)]
    pub input_filename: PathBuf,
}

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

#[derive(Debug, Default, PartialEq)]
struct AlmanachRange {
    source: usize,
    dest: usize,
    length: usize,
}

fn almanach_range<Input>() -> impl Parser<Input, Output = AlmanachRange>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        _usize().skip(spaces()),
        _usize().skip(spaces()),
        _usize().skip(spaces()),
    )
        .map(
            |(dest, source, length): (usize, usize, usize)| AlmanachRange {
                source,
                dest,
                length,
            },
        )
}

#[derive(Debug, Default)]
struct AlmanachMapping(Vec<AlmanachRange>);

fn almanach_mapping<Input>() -> impl Parser<Input, Output = AlmanachMapping>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        skip_until(char(':')).skip(char(':')).skip(char('\n')),
        sep_by(almanach_range(), spaces()),
    )
        .map(|((), ranges): ((), Vec<AlmanachRange>)| AlmanachMapping(ranges))
}

fn almanach<Input>() -> impl Parser<Input, Output = Vec<AlmanachMapping>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    many1(almanach_mapping()).map(|mapping: Vec<AlmanachMapping>| mapping)
}

impl AlmanachMapping {
    fn map(&self, seed: &usize) -> usize {
        for range in self.0.iter() {
            if (range.source..range.source + range.length).contains(seed) {
                let output = range.dest + (seed - range.source);
                return output;
            }
        }
        *seed
    }
}

fn seeds<Input>() -> impl Parser<Input, Output = Vec<usize>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (string("seeds: "), sep_by(_usize(), char(' '))).map(|(_, seeds): (_, Vec<usize>)| seeds)
}

pub fn compute(filename: PathBuf) {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut seed_line = String::new();
    reader.read_line(&mut seed_line).ok();
    let mut seeds_vec = seeds().easy_parse(&seed_line[..]).unwrap().0;

    reader.fill_buf().ok();
    let input: &str = std::str::from_utf8(reader.buffer()).unwrap();
    let almanach = almanach().easy_parse(input).unwrap().0;
    for almanach_mapping in almanach.iter() {
        for seed in seeds_vec.iter_mut() {
            *seed = almanach_mapping.map(seed);
        }
    }

    println!("Output:{:?}", seeds_vec.iter().min().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_almanach_mapping() {
        let input = "seed-to-soil map:
        50 98 2
        52 50 48";
        let output = almanach_mapping().easy_parse(input).unwrap();
        println!("Test1:{:?}", output);
    }

    #[test]
    fn test_seeds() {
        let input = "seeds: 79 14 55 13
        
        seed-to-soil map:
        50 52";
        let expected = vec![79, 14, 55, 13];
        let output = seeds().easy_parse(input).unwrap();
        assert_eq!(output.0, expected);
    }

    #[test]
    fn test_almanach() {
        let input = "seed-to-soil map:
        30 52 39
        40 90 2

        seed-to-soil map:
        22 52 8
        12 20 3";
        let expected = AlmanachRange {
            source: 52,
            dest: 22,
            length: 8,
        };
        let output = almanach().easy_parse(input).unwrap().0;

        assert_eq!(output[1].0[0], expected);
    }
}
