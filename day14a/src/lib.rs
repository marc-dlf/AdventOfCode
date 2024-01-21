use std::string::String;
use std::{collections::HashSet, hash::Hash};

use combine::{
    choice, many1,
    parser::char::{char, newline},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};
use nalgebra::{
    dmatrix, matrix, DMatrix, DMatrixView, DVectorView, Matrix, Matrix1, Matrix3, MatrixView1,
    Rotation2, VectorView1,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Symbol {
    Round,
    Cube,
    Empty,
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

pub fn get_weight(column: &DVectorView<Symbol>) -> usize {
    let mut total = 0;
    let mut previous_cube: usize = column.len();
    let mut cnt = 0;
    for (i, s) in column.iter().enumerate() {
        match s {
            Symbol::Cube => {
                previous_cube = column.len() - (i + 1);
                cnt = 0;
            }
            Symbol::Round => {
                total += previous_cube - cnt;
                cnt += 1;
            }
            Symbol::Empty => {}
        }
    }
    total
}

pub fn compute(input: &str) -> usize {
    let (platform, _) = platform().easy_parse(input).unwrap();
    let mut total = 0;
    for col in platform.0.column_iter() {
        total += get_weight(&col)
    }
    total
}

pub fn rotate(m: DMatrix<usize>) -> DMatrix<usize> {
    let t = m.transpose();
    DMatrix::from_row_iterator(
        t.nrows(),
        t.ncols(),
        t.row_iter()
            .flat_map(|v| v.into_iter().rev().copied().collect::<Vec<usize>>()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

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

        let (output, rest) = platform().easy_parse(input).unwrap();
        //println!("{:?}", output);
        assert!(rest.is_empty());
        assert_eq!(get_weight(&output.0.column(0)), 34);
        assert_eq!(get_weight(&output.0.column(2)), 17);
    }

    #[test]
    fn test_rotate() {
        let mut m = dmatrix![1,2,3;4,5,6;7,8,9;10,11,12];
        m = rotate(m);
        let expected = dmatrix![10,7,4,1;11,8,5,2;12,9,6,3];
        assert_eq!(m, expected);
    }
}
