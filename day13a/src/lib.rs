use std::string::String;
use std::{collections::HashSet, hash::Hash};

use combine::{
    choice, many1,
    parser::char::{char, digit, newline},
    sep_by1, sep_end_by1, EasyParser, ParseError, Parser, Stream,
};
use ndarray::{s, Array2, Axis};

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

#[derive(Debug)]
struct Island(Array2<bool>);

pub fn is_palindrome(row: &Vec<bool>) -> bool {
    let mut i = 0;
    while i < (row.len() / 2) {
        if row[i] != row[row.len() - 1 - i] {
            return false;
        }
        i += 1
    }
    true
}

pub fn compute(input: &str) -> usize {
    let (islands, _) = islands().easy_parse(input).unwrap();
    let mut total = 0;
    for island in islands.iter() {
        if let Some(v) = find_mirror(&island.0, true) {
            total += v;
        } else if let Some(v) = find_mirror(&island.0, false) {
            total += 100 * v;
        }
    }
    total
}

fn symbol<Input>() -> impl Parser<Input, Output = bool>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(char('.').map(|_| false), char('#').map(|_| true))
}

fn island<Input>() -> impl Parser<Input, Output = Island>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_end_by1(many1::<Vec<bool>, _, _>(symbol()), newline()),).map(
        |(rows,): (Vec<Vec<bool>>,)| {
            let mut data = Vec::new();
            let ncols = rows.first().unwrap().len();
            let mut nrows = 0;
            for row in rows.iter() {
                data.extend_from_slice(row);
                nrows += 1;
            }
            Island(Array2::from_shape_vec((nrows, ncols), data).unwrap())
        },
    )
}

pub fn find_mirror(grid: &Array2<bool>, row_direction: bool) -> Option<usize> {
    let (n1, n2) = match row_direction {
        true => (grid.dim().0, grid.dim().1),
        false => (grid.dim().1, grid.dim().0),
    };
    //println!("{:?} {:?}", n1, n2);
    let mut candidates: HashSet<usize> = (1..n2).collect();
    for i in 0..n1 {
        let pattern = match row_direction {
            true => grid.row(i),
            false => grid.column(i),
        };
        let mut to_remove: Vec<usize> = vec![];
        for &c in candidates.iter() {
            let size = std::cmp::min(c, n2 - c);
            let pattern = pattern
                .slice(s![c - size..c + size])
                .into_iter()
                .copied()
                .collect();
            if !is_palindrome(&pattern) {
                to_remove.push(c);
            }
            //println!("{:?} {:?} {:?}", i, &pattern, to_remove)
        }
        to_remove.iter().for_each(|v| {
            candidates.remove(v);
        });
    }
    candidates.iter().next().copied()
}

fn islands<Input>() -> impl Parser<Input, Output = Vec<Island>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by1(island(), newline()).map(|islands| islands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse_islands() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

...##......##.#
.#....#..##..#.
#.####.##.#....
##....##.#.#...
#.####.#######.
........#.#####
##.#..##....#.#
#.#..#.#.###...
#.#..#.#.###...

#.#.#.#.#
###.#.#.#
.##.#####
.##.#####
###.#.#.#
#.#.#.#.#
.#.##.#.#
#.#..#..#
..###.#..
#..#.#.#.
##.#...#.";
        let (output, rest) = islands().easy_parse(input).unwrap();
        //println!("{:?}", output);
        assert!(rest.is_empty());
        assert!(output[0].0[(0, 3)]);
        assert!(!output[0].0[(0, 1)]);
        assert!(output[1].0[(0, 0)]);

        //test mirror
        assert_eq!(find_mirror(&output[0].0, true), Some(5));
        assert_eq!(find_mirror(&output[1].0, true), None);
        assert_eq!(find_mirror(&output[1].0, false), Some(4));
        assert_eq!(find_mirror(&output[2].0, false), Some(8));
        assert_eq!(find_mirror(&output[2].0, true), None);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&vec![true, false, false, true]));
        assert!(!is_palindrome(&vec![true, false, true, true]));
    }
}
