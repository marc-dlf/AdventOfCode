use anyhow::anyhow;
use anyhow::Result;

use combine::{
    choice, many1,
    parser::char::{char, newline},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};

use ndarray::{Array2, ArrayView};

#[derive(Default, Clone, PartialEq, Eq, Debug)]
enum Tile {
    #[default]
    Ground,
    Start,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl From<(isize, isize)> for Direction {
    fn from(item: (isize, isize)) -> Self {
        match item {
            (-1, 0) => Direction::N,
            (1, 0) => Direction::S,
            (0, -1) => Direction::W,
            (0, 1) => Direction::E,
            _ => unreachable!("Not a valid direction!"),
        }
    }
}

fn revert(d: Direction) -> Direction {
    match d {
        Direction::N => Direction::S,
        Direction::E => Direction::W,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
    }
}

#[derive(Debug)]
struct Maze {
    grid: Array2<Tile>,
    start: (usize, usize),
}

impl Maze {
    fn next(
        &self,
        incoming_direction: Direction,
        pos: (usize, usize),
    ) -> Result<(Direction, (usize, usize))> {
        match (&self.grid[pos], incoming_direction) {
            // NS
            (Tile::NS, Direction::S) => Ok((Direction::S, (pos.0 + 1, pos.1))),
            (Tile::NS, Direction::N) => Ok((Direction::N, (pos.0 - 1, pos.1))),
            (Tile::NS, d) => Err(anyhow!("Tile NS / Forbidden direction: {:?}", d)),
            //EW
            (Tile::EW, Direction::E) => Ok((Direction::E, (pos.0, pos.1 + 1))),
            (Tile::EW, Direction::W) => Ok((Direction::W, (pos.0, pos.1 - 1))),
            (Tile::EW, d) => Err(anyhow!("Tile EW / Forbidden direction: {:?}", d)),
            //NE
            (Tile::NE, Direction::S) => Ok((Direction::E, (pos.0, pos.1 + 1))),
            (Tile::NE, Direction::W) => Ok((Direction::N, (pos.0 - 1, pos.1))),
            (Tile::NE, d) => Err(anyhow!("Tile NE / Forbidden direction: {:?}", d)),
            //NW
            (Tile::NW, Direction::S) => Ok((Direction::W, (pos.0, pos.1 - 1))),
            (Tile::NW, Direction::E) => Ok((Direction::N, (pos.0 - 1, pos.1))),
            (Tile::NW, d) => Err(anyhow!("Tile NW / Forbidden direction: {:?}", d)),
            //SE
            (Tile::SE, Direction::W) => Ok((Direction::S, (pos.0 + 1, pos.1))),
            (Tile::SE, Direction::N) => Ok((Direction::E, (pos.0, pos.1 + 1))),
            (Tile::SE, d) => Err(anyhow!("Tile SE / Forbidden direction: {:?}", d)),
            //EW
            (Tile::SW, Direction::E) => Ok((Direction::S, (pos.0 + 1, pos.1))),
            (Tile::SW, Direction::N) => Ok((Direction::W, (pos.0, pos.1 - 1))),
            (Tile::SW, d) => Err(anyhow!("Tile SW / Forbidden direction: {:?}", d)),
            //Start
            (Tile::Start, _) => Ok(self.loop_entry()),
            //Errors
            (Tile::Ground, _) => Err(anyhow!("Can't reach ground!")),
        }
    }

    fn loop_entry(&self) -> (Direction, (usize, usize)) {
        let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let neighbors = directions
            .iter()
            .map(|(i, j)| (self.start.0 as isize + i, self.start.1 as isize + j));

        for neighbor in neighbors {
            if !self.is_oob(neighbor) {
                for (i, j) in directions.iter() {
                    if let Ok((incoming_dir, prev_pos)) = self.next(
                        Direction::from((*i, *j)),
                        (neighbor.0 as usize, neighbor.1 as usize),
                    ) {
                        if prev_pos == self.start {
                            return (
                                revert(incoming_dir),
                                (neighbor.0 as usize, neighbor.1 as usize),
                            );
                        }
                    }
                }
            }
        }
        unreachable!("No valid entrypoint was found!");
    }

    fn is_oob(&self, pos: (isize, isize)) -> bool {
        if (pos.0 < 0)
            | (pos.1 < 0)
            | (pos.0 as usize >= self.grid.dim().0)
            | (pos.1 as usize >= self.grid.dim().1)
        {
            return true;
        }
        false
    }
}

fn tile<Input>() -> impl Parser<Input, Output = Tile>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice!(
        char('|').map(|_| Tile::NS),
        char('-').map(|_| Tile::EW),
        char('L').map(|_| Tile::NE),
        char('J').map(|_| Tile::NW),
        char('7').map(|_| Tile::SW),
        char('F').map(|_| Tile::SE),
        char('.').map(|_| Tile::Ground),
        char('S').map(|_| Tile::Start)
    )
}

fn maze<Input>(width: usize) -> impl Parser<Input, Output = Maze>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by1(many1(tile()), newline()),).map(move |(rows,): (Vec<Vec<Tile>>,)| {
        let mut i = 0;
        let mut grid: Array2<Tile> = Array2::default((0, width));
        let mut start: Option<(usize, usize)> = None;

        rows.into_iter().for_each(|row| {
            if let Some(j) = row.iter().position(|tile| tile == &Tile::Start) {
                start = Some((i, j));
            }
            grid.push_row(ArrayView::from(&row)).unwrap();
            i += 1
        });
        match start {
            None => unreachable!("Start not found!"),
            Some(start) => Maze { grid, start },
        }
    })
}

pub fn compute(input: &str) -> usize {
    let width = &input.lines().next().unwrap().len();
    let (maze, _) = maze(*width).easy_parse(input).unwrap();
    let mut cnt = 1;
    let (mut direction, mut pos) = maze.loop_entry();

    while pos != maze.start {
        (direction, pos) = maze.next(direction, pos).unwrap();
        cnt += 1
    }
    cnt / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;
    use ndarray::{array, s, Array, ArrayView};

    #[test]
    fn test_ndarray() {
        // create an empty array and append
        let mut a = Array::zeros((0, 4));
        a.push_row(ArrayView::from(&[1., 2., 3., 4.])).unwrap();
        a.push_row(ArrayView::from(&[-1., -2., -3., -4.])).unwrap();

        assert_eq!(a, array![[1., 2., 3., 4.], [-1., -2., -3., -4.]]);
    }

    #[test]
    fn test_parse_maze() {
        //input 1
        let input = ".....
.F-7.
.S.|.
.L-J.
.....";
        let width = &input.lines().next().unwrap().len();
        assert_eq!(*width, 5);
        let (maze, rest) = maze(*width).easy_parse(input).unwrap();
        assert_eq!(maze.start, (2, 1));
        assert_eq!(
            maze.grid.slice(s![1, ..]),
            ArrayView::from(&[Tile::Ground, Tile::SE, Tile::EW, Tile::SW, Tile::Ground])
        );
        assert!(rest.is_empty());
        let accepted_values = vec![(Direction::N, (1, 1)), (Direction::S, (3, 1))];
        println!("Loop entry {:?}", &maze.loop_entry());
        assert!(accepted_values.contains(&maze.loop_entry()));
    }

    #[test]
    fn test_compute() {
        //input 1
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let res = compute(input);
        assert_eq!(res, 8);
    }
}
