use std::collections::HashMap;
use std::string::String;

use combine::{
    choice, many1,
    parser::char::{char, digit, newline},
    sep_by1, sep_end_by1, EasyParser, ParseError, Parser, Stream,
};
use ndarray::{s, Array2, ArrayView1};

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

pub fn get_pattern(line: &ArrayView1<bool>, mirror_idx: usize) -> Vec<bool> {
    let size = std::cmp::min(mirror_idx, line.len() - mirror_idx);
    line.slice(s![mirror_idx - size..mirror_idx + size])
        .into_iter()
        .copied()
        .collect()
}

pub fn find_mirror(grid: &Array2<bool>, vertical_mirror: bool) -> Option<usize> {
    // search direction is the direction along which the mirror will be
    // iter_n corresponds to the number of times we will search for mirrors
    let (iter_n, search_n) = match vertical_mirror {
        true => (grid.dim().0, grid.dim().1), // if the mirror is vertical, we iterate over rows
        false => (grid.dim().1, grid.dim().0), //if the mirror is horizontal we iterate over columns
    };
    let mut candidates: HashMap<usize, (usize, Option<usize>)> =
        (1..search_n).map(|v| (v, (1, None))).collect();

    //get candidates : mirror positions where exactly 1 row/column is problematic
    for i in 0..iter_n {
        let line = match vertical_mirror {
            true => grid.row(i),
            false => grid.column(i),
        };
        let mut to_remove: Vec<usize> = vec![];
        for (&mirror_idx, v) in candidates.iter_mut() {
            let pattern = get_pattern(&line, mirror_idx);
            if !is_palindrome(&pattern) {
                match v.0 {
                    1 => {
                        v.0 -= 1;
                        v.1 = Some(i);
                    }
                    0 => {
                        to_remove.push(mirror_idx);
                    }
                    _ => unreachable!(),
                }
            }
        }
        to_remove.iter().for_each(|v| {
            candidates.remove(v);
        });
    }

    // find fitting candidate
    for (&mirror_idx, &(_, pos)) in candidates.iter() {
        if let Some(i) = pos {
            let line = match vertical_mirror {
                true => grid.row(i),
                false => grid.column(i),
            };
            let mut pattern = get_pattern(&line, mirror_idx);
            for k in 0..pattern.len() {
                pattern[k] = !pattern[k];
                if is_palindrome(&pattern) {
                    return Some(mirror_idx);
                }
                pattern[k] = !pattern[k];
            }
        }
    }
    None
}

//candidates.iter().next().copied()

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
#....#..#";
        let (output, rest) = islands().easy_parse(input).unwrap();
        assert!(rest.is_empty());
        assert!(output[0].0[(0, 3)]);
        assert!(!output[0].0[(0, 1)]);
        assert!(output[1].0[(0, 0)]);

        //test mirror
        assert_eq!(find_mirror(&output[0].0, true), None);
        assert_eq!(find_mirror(&output[0].0, false), Some(3));
        assert_eq!(find_mirror(&output[1].0, false), Some(1));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&vec![true, false, false, true]));
        assert!(!is_palindrome(&vec![true, false, true, true]));
    }
}
