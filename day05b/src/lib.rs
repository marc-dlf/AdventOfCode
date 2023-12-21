use std::cmp::{max, min, Ordering};
use std::collections::HashSet;
use std::ops::Range;

use combine::EasyParser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use combine::{
    many1,
    parser::char::{char, digit, space, spaces, string},
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

impl AlmanachRange {
    pub fn source_range(&self) -> Range<usize> {
        self.source..self.source + self.length
    }

    pub fn dest_range(&self) -> Range<usize> {
        self.dest..self.dest + self.length
    }
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
        skip_until(char(':')).skip(char(':')).skip(spaces()),
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Seed(Range<usize>);

impl Seed {
    fn new(start: usize, length: usize) -> Self {
        Self(start..start + length)
    }

    fn overlap(&self, range: &Range<usize>) -> bool {
        max(self.0.start, range.start) < min(self.0.end, range.end)
    }

    fn contains(&self, range: &Range<usize>) -> bool {
        (self.0.start <= range.start) & (range.end <= self.0.end)
    }

    fn convert(&self, range: &AlmanachRange) -> (Option<Seed>, Vec<Seed>) {
        let seed_range = self.0.start..self.0.end;
        let source_range = range.source_range();
        let dest_range = range.dest_range();

        let mut new_seeds = match !self.overlap(&source_range) {
            // Non overlapping intervals
            // Examples : (0..5) & (8..10) ; (3..7) & (7..10)
            true => (None, vec![self.clone()]),
            //false => match seed_range.start >= source_range.start {
            false => {
                match self.contains(&source_range) {
                    // source range is included in seed range
                    true => {
                        let offset = source_range.start - seed_range.start;
                        let len_source = source_range.end - source_range.start;
                        (
                            Some(Seed(
                                dest_range.start + offset..dest_range.start + offset + len_source,
                            )),
                            vec![
                                Seed(seed_range.start..source_range.start),
                                Seed(source_range.end..seed_range.end),
                            ],
                        )
                    }
                    // source range is not included in seed range
                    false => {
                        match seed_range.start > source_range.start {
                            //overlap on the left part of seed range
                            true => {
                                let offset = seed_range.start - source_range.start;
                                let mapped_len = match seed_range.end >= source_range.end {
                                    true => source_range.end - seed_range.start,
                                    false => seed_range.end - seed_range.start,
                                };
                                (
                                    Some(Seed(
                                        dest_range.start + offset
                                            ..dest_range.start + offset + mapped_len,
                                    )),
                                    vec![Seed(source_range.end..seed_range.end)],
                                )
                            }
                            //overlap on the right part of the seed range
                            false => {
                                // case when end of seed index overlap with start of source index
                                // Example: seed (0..5) & source (3..10) because 0<3
                                (
                                    // in source -> transfo
                                    Some(Seed(
                                        dest_range.start
                                            ..dest_range.start
                                                + (seed_range.end - source_range.start),
                                    )),
                                    // out of source range -> no transfo
                                    vec![Seed(seed_range.start..source_range.start)],
                                )
                            }
                        }
                    }
                }
            }
        };

        new_seeds.0 = match new_seeds.0 {
            Some(s) => match s.0.start < s.0.end {
                true => Some(s),
                false => None,
            },
            _ => None,
        };
        new_seeds.1.retain(|s| s.0.start < s.0.end);
        new_seeds
    }
}

impl PartialOrd for Seed {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Seed {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.start.cmp(&other.0.start)
    }
}

fn seed<Input>() -> impl Parser<Input, Output = Seed>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (_usize(), space(), _usize())
        .map(|(start, _, length): (usize, _, usize)| Seed::new(start, length))
}

fn seeds<Input>() -> impl Parser<Input, Output = Vec<Seed>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (string("seeds: "), sep_by(seed(), char(' '))).map(|(_, seeds): (_, Vec<Seed>)| seeds)
}

pub fn compute(filename: PathBuf) {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut seed_line = String::new();
    reader.read_line(&mut seed_line).ok();
    let mut seeds_hashset: HashSet<Seed> =
        HashSet::from_iter(seeds().easy_parse(&seed_line[..]).unwrap().0);

    reader.fill_buf().ok();
    let input: &str = std::str::from_utf8(reader.buffer()).unwrap();
    let almanach = almanach().easy_parse(input).unwrap().0;
    for mapping in almanach.iter() {
        let mut transformed_seeds: HashSet<Seed> = HashSet::new();

        for almanach_range in mapping.0.iter() {
            let current_seeds: Vec<Seed> = seeds_hashset.clone().into_iter().collect();

            for s in current_seeds.iter() {
                seeds_hashset.remove(s);
                let (new_seed, remainder) = s.convert(almanach_range);
                if let Some(new_seed) = new_seed {
                    transformed_seeds.insert(new_seed);
                }
                let tmp_hashset: HashSet<Seed> = HashSet::from_iter(remainder.into_iter());
                seeds_hashset.extend(tmp_hashset);
            }
        }

        seeds_hashset.extend(transformed_seeds.clone());
    }
    println!("Output:{:?}", seeds_hashset.iter().min().unwrap());
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
    }

    #[test]
    fn test_seeds() {
        let input = "seeds: 79 14 55 13";
        let seed1 = Seed::new(79, 14);
        let seed2 = Seed::new(55, 13);
        let expected = vec![seed1, seed2];
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

    #[test]
    fn test_seed_convert() {
        let seed = Seed::new(5, 5);

        // no overlap 1
        let almanach_1 = AlmanachRange {
            source: 11,
            dest: 2,
            length: 2,
        };
        assert_eq!(
            seed.convert(&almanach_1),
            (None, vec![Seed(5..10)]),
            "Input 1"
        );

        // no overlap 2
        let almanach_2 = AlmanachRange {
            source: 3,
            dest: 2,
            length: 2,
        };
        assert_eq!(
            seed.convert(&almanach_2),
            (None, vec![Seed(5..10)]),
            "Input 2"
        );

        // overlap 1
        let almanach_3 = AlmanachRange {
            source: 3,
            dest: 20,
            length: 4,
        };
        assert_eq!(
            seed.convert(&almanach_3),
            (Some(Seed(22..24)), vec![Seed(7..10)]),
            "Input 3"
        );

        // // overlap 2
        let almanach_4 = AlmanachRange {
            source: 8,
            dest: 20,
            length: 4,
        };
        assert_eq!(
            seed.convert(&almanach_4),
            (Some(Seed(20..22)), vec![Seed(5..8)]),
            "Input 4"
        );

        // overlap all
        let almanach_5 = AlmanachRange {
            source: 0,
            dest: 20,
            length: 15,
        };
        assert_eq!(
            seed.convert(&almanach_5),
            (Some(Seed(25..30)), vec![]),
            "Input 5"
        );

        // overlap included
        let almanach_6 = AlmanachRange {
            source: 6,
            dest: 20,
            length: 2,
        };
        assert_eq!(
            seed.convert(&almanach_6),
            (Some(Seed(21..23)), vec![Seed(5..6), Seed(8..10)]),
            "Input 6"
        );
    }
}
