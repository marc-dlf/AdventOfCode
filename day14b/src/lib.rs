use std::collections::HashSet;

use combine::{
    choice, many1,
    parser::char::{char, newline},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};
use nalgebra::{DMatrix, Scalar};

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Symbol {
    Round,
    Cube,
    Empty,
}

impl std::fmt::Display for Symbol {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Cube => write!(f, "#"),
            Symbol::Round => write!(f, "O"),
            Symbol::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct Platform(DMatrix<Symbol>);

fn symbol<Input>() -> impl Parser<Input, Output = Symbol>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(
        char('.').map(|_| Symbol::Empty),
        char('#').map(|_| Symbol::Cube),
        char('O').map(|_| Symbol::Round)
    )
}

fn platform<Input>() -> impl Parser<Input, Output = Platform>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by1(many1::<Vec<Symbol>, _, _>(symbol()), newline())).map(|rows: Vec<Vec<Symbol>>| {
        let mut data = Vec::new();
        let ncols = rows.first().unwrap().len();
        let mut nrows = 0;
        for row in rows.iter() {
            data.extend_from_slice(row);
            nrows += 1;
        }
        Platform(DMatrix::from_row_iterator(nrows, ncols, data))
    })
}

impl Platform {
    pub fn move_north(&mut self) {
        let mut round_new_pos: HashSet<(usize, usize)> = HashSet::new();
        //new pos
        for (j, col) in self.0.column_iter().enumerate() {
            let mut previous_cube: usize = col.len();
            let mut cnt = 0;
            for (i, s) in col.iter().enumerate() {
                match s {
                    Symbol::Cube => {
                        previous_cube = col.len() - (i + 1);
                        cnt = 0;
                    }
                    Symbol::Round => {
                        let north_load = previous_cube - cnt;
                        round_new_pos.insert((col.len() - north_load, j));
                        cnt += 1;
                    }
                    Symbol::Empty => {}
                }
            }
        }
        //update platform
        for i in 0..self.0.nrows() {
            for j in 0..self.0.ncols() {
                match self.0[(i, j)] {
                    Symbol::Cube => {}
                    _ => {
                        if round_new_pos.contains(&(i, j)) {
                            self.0[(i, j)] = Symbol::Round;
                        } else {
                            self.0[(i, j)] = Symbol::Empty;
                        }
                    }
                }
            }
        }
    }

    pub fn cycle(&mut self, repeat: usize) {
        let mut pos_history: Vec<DMatrix<Symbol>> = vec![];
        for i in 0..repeat {
            if let Some(j) = pos_history.iter().position(|p| p == &self.0) {
                let cycle_length = i - j;
                let remaining = (repeat - i) % cycle_length;
                println!("Cycle found! First occurence of position : {}", j);
                println!("Current occurence of position: {i}. Length of cycle : {cycle_length}",);
                println!("Skipping cycles. Final position is the {remaining} in the cycle");
                self.cycle(remaining);
                return;
            }
            pos_history.push(self.0.clone());
            for _ in 0..4 {
                self.move_north();
                self.0 = rotate(&self.0);
            }
        }
    }

    pub fn compute_support(&self) -> usize {
        let mut total = 0;
        for i in 0..self.0.nrows() {
            for j in 0..self.0.ncols() {
                if let Symbol::Round = self.0[(i, j)] {
                    total += self.0.nrows() - i;
                }
            }
        }
        total
    }
}

pub fn compute(input: &str) -> usize {
    let (mut platform, _) = platform().easy_parse(input).unwrap();
    platform.cycle(1000000000);
    platform.compute_support()
}

pub fn rotate<T>(m: &DMatrix<T>) -> DMatrix<T>
where
    T: Scalar + Copy,
{
    let t = m.transpose();
    DMatrix::from_row_iterator(
        t.nrows(),
        t.ncols(),
        t.row_iter()
            .flat_map(|v| v.into_iter().rev().copied().collect::<Vec<T>>()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;
    use nalgebra::dmatrix;

    #[test]
    fn test_parse_islands() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let (mut output, rest) = platform().easy_parse(input).unwrap();
        assert!(rest.is_empty());
        output.cycle(1000000000);
        assert_eq!(output.compute_support(), 64);
    }

    #[test]
    fn test_rotate() {
        let mut m = dmatrix![1,2,3;4,5,6;7,8,9;10,11,12];
        m = rotate(&m);
        let expected = dmatrix![10,7,4,1;11,8,5,2;12,9,6,3];
        assert_eq!(m, expected);
    }
}
