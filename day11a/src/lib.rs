use std::string::String;

//use ::polyfit_rs::polyfit_rs::polyfit;
use combine::{
    choice, many1,
    parser::char::{char, digit, newline},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};
use ndarray::{Array2, Axis};

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

#[derive(Debug)]
struct Image(Array2<bool>);

fn manhattan(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

impl Image {
    fn get_galaxy_indexes(&self, axis: Axis) -> Vec<usize> {
        self.0
            .lanes(axis)
            .into_iter()
            .enumerate()
            .filter(|(_, galaxy)| galaxy.iter().all(|&x| !x))
            .map(|(i, _)| i)
            .collect()
    }

    fn get_true_position(
        &self,
        pos: (usize, usize),
        galaxy_rows: &[usize],
        galaxy_cols: &[usize],
        distortion: usize,
    ) -> (usize, usize) {
        (
            pos.0
                + (galaxy_rows
                    .iter()
                    .filter(|&&x| x < pos.0)
                    .collect::<Vec<_>>()
                    .len()
                    * (distortion - 1)),
            pos.1
                + (galaxy_cols
                    .iter()
                    .filter(|&&x| x < pos.1)
                    .collect::<Vec<_>>()
                    .len()
                    * (distortion - 1)),
        )
    }

    fn compute_shortest_paths(&self, distortion: usize) -> usize {
        //rows containing no galaxy
        let galaxy_cols = self.get_galaxy_indexes(Axis(0));
        //columns containing no galaxy
        let galaxy_rows = self.get_galaxy_indexes(Axis(1));
        //galaxies positions
        let mut galaxy_pos: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.0.dim().0 {
            for j in 0..self.0.dim().1 {
                let pos = (i, j);
                if self.0[pos] {
                    galaxy_pos.push(self.get_true_position(
                        pos,
                        &galaxy_rows,
                        &galaxy_cols,
                        distortion,
                    ));
                }
            }
        }
        let mut cnt: usize = 0;
        while galaxy_pos.len() > 1 {
            let pos = galaxy_pos.pop().unwrap();
            cnt += galaxy_pos
                .iter()
                .map(|other| manhattan(pos, *other))
                .sum::<usize>();
        }
        cnt
    }
}

pub fn compute(input: &str) -> usize {
    let (image, _) = image().easy_parse(input).unwrap();
    //Part 1
    //image.compute_shortest_paths(2)
    //Part 2
    image.compute_shortest_paths(1000000)
}

fn image<Input>() -> impl Parser<Input, Output = Image>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by1(many1::<Vec<bool>, _, _>(symbol()), newline()),).map(
        |(galaxies,): (Vec<Vec<bool>>,)| {
            let mut data = Vec::new();
            let ncols = galaxies.first().unwrap().len();
            let mut nrows = 0;
            for galaxy in galaxies.iter() {
                data.extend_from_slice(galaxy);
                nrows += 1;
            }
            Image(Array2::from_shape_vec((nrows, ncols), data).unwrap())
        },
    )
}

fn symbol<Input>() -> impl Parser<Input, Output = bool>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(char('.').map(|_| false), char('#').map(|_| true))
}

//pub fn compute(input: &str) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse_image() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let (mut output, rest) = image().easy_parse(input).unwrap();
        println!("{:?}", output);
        assert!(rest.is_empty());
        assert_eq!(output.compute_shortest_paths(2), 374);
    }
}
