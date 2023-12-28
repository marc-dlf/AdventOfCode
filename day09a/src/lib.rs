use std::string::String;

//use ::polyfit_rs::polyfit_rs::polyfit;
use combine::{
    choice, many1,
    parser::char::{char, digit, newline},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};
//use polyfit_rs::polyfit_rs;

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

fn _isize<Input>() -> impl Parser<Input, Output = isize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(
        _usize().map(|n| n.try_into().unwrap()),
        (char('-'), _usize()).map(|(_, n)| {
            let n: isize = n.try_into().unwrap();
            -n
        })
    )
}

// pub fn compute_next(measures: &[isize]) -> f64 {
//     let measures: Vec<f64> = measures.iter().map(|x| *x as f64).collect();
//     let x: Vec<f64> = (0..measures.len()).map(|x| x as f64).collect();
//     let poly = polyfit(&x[0..9], &measures[0..9], 8).unwrap();
//     println!("{:?}", measures.len());
//     extrapolate(poly, measures.len() as f64)
// }

// pub fn extrapolate(poly: Vec<f64>, x: f64) -> f64 {
//     let mut res = 0.;
//     for (i, f) in poly.iter().enumerate() {
//         res += f * x.powi(i.try_into().unwrap());
//     }
//     res
// }

pub fn next_row(row: Vec<isize>) -> (Vec<isize>, bool) {
    let mut differences: Vec<isize> = vec![];
    let mut finish: bool = true;
    for i in 0..(row.len() - 1) {
        let diff = row[i + 1] - row[i];
        if diff != 0 {
            finish = false;
        }
        differences.push(diff);
    }
    (differences, finish)
}

//Part 1
// pub fn extrapolate(row: Vec<isize>) -> isize {
//     let mut finish = false;
//     let mut last_values: Vec<isize> = vec![];
//     let mut res: isize = *row.last().unwrap();
//     let mut current_row = row;

//     while !finish {
//         (current_row, finish) = next_row(current_row);
//         last_values.push(*current_row.last().unwrap());
//     }

//     res += last_values.iter().sum::<isize>();
//     res
// }

//Part 2
pub fn extrapolate(row: Vec<isize>) -> isize {
    let mut finish = false;
    let mut last_values: Vec<isize> = vec![];
    let first: isize = *row.first().unwrap();
    let mut current_row = row;

    while !finish {
        (current_row, finish) = next_row(current_row);
        last_values.push(*current_row.first().unwrap());
    }

    last_values = last_values.into_iter().collect();
    let mut res = 0;
    for v in last_values.iter().rev() {
        res = v - res;
    }
    first - res
}

pub fn compute(input: &str) -> isize {
    let mut res: isize = 0;
    let oasis_measures = measures().easy_parse(input).unwrap().0;
    oasis_measures.into_iter().for_each(|measures| {
        res += extrapolate(measures);
    });
    res
}

fn measures<Input>() -> impl Parser<Input, Output = Vec<Vec<isize>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by1(sep_by1(_isize(), char(' ')), newline()),)
        .map(|(measures,): (Vec<Vec<isize>>,)| measures)
}

//pub fn compute(input: &str) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;
    use polyfit_rs::polyfit_rs;

    #[test]
    fn test_polyfit() {
        let x = [0., 1., 2., 3., 4.];
        let y = [1., 4., 7., 10., 13.];
        let rs = polyfit_rs::polyfit(&x, &y, 3).unwrap();
        println!("{:?}", rs);

        fn extrapolate(poly: Vec<f64>, x: f64) -> f64 {
            let mut res = 0.;
            for (i, f) in poly.iter().enumerate() {
                res += f * x.powi(i.try_into().unwrap());
            }
            res
        }
        println!("{:?}", extrapolate(rs, 5.));
        // assert!(false);
    }

    #[test]
    fn test_parse_measures() {
        let input = "-7 -13 -20 -17 16 108
-9 -7 2 18 41 71";
        let (output, rest) = measures().easy_parse(input).unwrap();
        assert_eq!(
            output,
            vec![
                vec![-7, -13, -20, -17, 16, 108],
                vec![-9, -7, 2, 18, 41, 71]
            ]
        );
        assert!(rest.is_empty());
    }

    #[test]
    fn test_rounding() {
        //let f1: f64 = -15667.999998316609;
        let f2: f64 = 413948.00000249385;
        let f3: f64 = 68151.99999870891;

        //assert_eq!(f1.round() as i64, -15666);
        assert_eq!(f2.round() as i64, 413948);
        assert_eq!(f3.round() as i64, 68152);
    }

    #[test]
    fn test_extrapolate_part2() {
        let input = vec![10, 13, 16, 21, 30, 45];
        let output = extrapolate(input);

        assert_eq!(output, 5);
    }
}
