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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
enum State {
    #[default]
    Unknown,
    Wall,
    Inside,
    Outside,
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

impl From<Direction> for (isize, isize) {
    fn from(val: Direction) -> Self {
        match val {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::W => (0, -1),
            Direction::E => (0, 1),
            _ => unreachable!("Not a valid direction!"),
        }
    }
}

impl Direction {
    fn parralel(&self, other: &Direction) -> bool {
        let d1 = [Direction::N, Direction::S];
        let d2 = [Direction::E, Direction::W];
        if ((d1.contains(self)) && (d1.contains(other)))
            | ((d2.contains(self)) && (d2.contains(other)))
        {
            return true;
        }
        false
    }
}

impl From<[Direction; 2]> for Tile {
    fn from(item: [Direction; 2]) -> Self {
        if item.contains(&Direction::N) && item.contains(&Direction::S) {
            return Tile::NS;
        }
        if item.contains(&Direction::N) && item.contains(&Direction::E) {
            return Tile::NE;
        }
        if item.contains(&Direction::N) && item.contains(&Direction::W) {
            return Tile::NW;
        }
        if item.contains(&Direction::S) && item.contains(&Direction::E) {
            return Tile::SE;
        }
        if item.contains(&Direction::S) && item.contains(&Direction::W) {
            return Tile::SW;
        }
        if item.contains(&Direction::E) && item.contains(&Direction::W) {
            return Tile::EW;
        }
        unreachable!("Impossible Tile from directions : {:?}", item);
    }
}

impl Tile {
    fn perpendicular(&self) -> Vec<Direction> {
        match self {
            Tile::NS => vec![Direction::E],
            Tile::EW => vec![Direction::S],
            Tile::NE => vec![Direction::S, Direction::W],
            Tile::NW => vec![Direction::S, Direction::E],
            Tile::SW => vec![Direction::N, Direction::E],
            Tile::SE => vec![Direction::N, Direction::W],
            _ => unreachable!("Perpendicular problem"),
        }
    }

    fn next_perpendicular(&self, p: Direction) -> Direction {
        match (self, p) {
            // No direction change
            (Tile::NS, p) => p,
            (Tile::EW, p) => p,
            // Direction change NE
            (Tile::NE, Direction::N) => Direction::E,
            (Tile::NE, Direction::S) => Direction::W,
            (Tile::NE, Direction::E) => Direction::N,
            (Tile::NE, Direction::W) => Direction::S,
            // Direction change NW
            (Tile::NW, Direction::N) => Direction::W,
            (Tile::NW, Direction::S) => Direction::E,
            (Tile::NW, Direction::E) => Direction::S,
            (Tile::NW, Direction::W) => Direction::N,
            // Direction change SE
            (Tile::SE, Direction::N) => Direction::W,
            (Tile::SE, Direction::S) => Direction::E,
            (Tile::SE, Direction::E) => Direction::S,
            (Tile::SE, Direction::W) => Direction::N,
            // Direction change SW
            (Tile::SW, Direction::N) => Direction::E,
            (Tile::SW, Direction::S) => Direction::W,
            (Tile::SW, Direction::E) => Direction::N,
            (Tile::SW, Direction::W) => Direction::S,
            // Start cell don't change
            (Tile::Start, p) => p,
            // Not possible
            e => unreachable!("Next Perpendicular problem:{:?}", e),
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
    state: Array2<State>,
    start: (usize, usize),
    start_tile: Option<Tile>,
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
            (Tile::Start, _) => {
                let (_, direction, pos) = self.loop_entry();
                Ok((direction, pos))
            }
            //Errors
            (Tile::Ground, _) => Err(anyhow!("Can't reach ground!")),
        }
    }

    fn loop_entry(&self) -> (Tile, Direction, (usize, usize)) {
        let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let neighbors = directions
            .iter()
            .map(|(i, j)| (self.start.0 as isize + i, self.start.1 as isize + j));
        let mut start_dirs: Vec<Direction> = vec![];

        for neighbor in neighbors {
            if !self.is_oob(neighbor) {
                for (i, j) in directions.iter() {
                    if let Ok((incoming_dir, prev_pos)) = self.next(
                        Direction::from((*i, *j)),
                        (neighbor.0 as usize, neighbor.1 as usize),
                    ) {
                        if prev_pos == self.start {
                            start_dirs.push(revert(incoming_dir));
                            if start_dirs.len() == 2 {
                                return (
                                    Tile::from([
                                        *start_dirs.first().unwrap(),
                                        *start_dirs.last().unwrap(),
                                    ]),
                                    revert(incoming_dir),
                                    (neighbor.0 as usize, neighbor.1 as usize),
                                );
                            }
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

    fn init_state(&mut self) {
        let (start_tile, mut direction, mut pos) = self.loop_entry();
        let mut rightmost: ((usize, usize), (Direction, Direction)) =
            ((0, 0), (Direction::N, Direction::N));

        self.start_tile = Some(start_tile);
        // Mark cells around starting point
        let first_perpendiculars = self.grid[pos].perpendicular();
        let mut perpendicular: Direction = *first_perpendiculars.first().unwrap();
        for p in first_perpendiculars.into_iter() {
            self.mark(pos, p);
            if p.parralel(&direction) {
                perpendicular = p;
            }
        }
        // Finish loop while keeping perpendicular direction consistent
        while pos != self.start {
            self.state[pos] = State::Wall;
            if pos.1 >= rightmost.0 .1 {
                rightmost = (
                    pos,
                    (
                        perpendicular,
                        self.grid[pos].next_perpendicular(perpendicular),
                    ),
                );
            }
            (direction, pos) = self.next(direction, pos).unwrap();

            self.mark(pos, perpendicular);
            perpendicular = self.grid[pos].next_perpendicular(perpendicular);
            self.mark(pos, perpendicular);
        }
        self.mark(pos, perpendicular);
        self.mark(
            pos,
            self.start_tile
                .as_ref()
                .unwrap()
                .next_perpendicular(perpendicular),
        );
        if pos.1 >= rightmost.0 .1 {
            rightmost = (
                pos,
                (
                    perpendicular,
                    self.start_tile
                        .as_ref()
                        .unwrap()
                        .next_perpendicular(perpendicular),
                ),
            );
        }
        if (rightmost.1 .0 == Direction::W) | (rightmost.1 .1 == Direction::W) {
            self.invert_state()
        }
    }

    fn floodfill(&mut self) {
        let mut q: Vec<(usize, usize)> = vec![];
        let mut state = State::Unknown;

        for i in 0..self.state.dim().0 {
            for j in 0..self.state.dim().1 {
                if (self.state[(i, j)] == State::Inside) | (self.state[(i, j)] == State::Outside) {
                    q.push((i, j));
                    state = self.state[(i, j)].clone();
                }
            }
        }

        while let Some(pos) = q.pop() {
            let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            let neighbors = directions
                .iter()
                .map(|(i, j)| (pos.0 as isize + i, pos.1 as isize + j));
            for n in neighbors {
                if !self.is_oob(n) {
                    let n: (usize, usize) = (n.0 as usize, n.1 as usize);
                    if self.state[n] == State::Unknown {
                        self.state[n] = state.clone();
                        q.push(n);
                    }
                }
            }
        }
    }

    fn invert_state(&mut self) {
        for elt in self.state.iter_mut() {
            match elt {
                State::Outside => {
                    *elt = State::Inside;
                }
                State::Inside => {
                    *elt = State::Outside;
                }
                State::Unknown => {}
                State::Wall => {}
            }
        }
    }

    fn mark(&mut self, wall_pos: (usize, usize), direction: Direction) {
        self.state[wall_pos] = State::Wall;
        let default_state = State::Outside;
        let direction: (isize, isize) = direction.into();
        let pos: (isize, isize) = (wall_pos.0 as isize, wall_pos.1 as isize);
        let pos = (pos.0 + direction.0, pos.1 + direction.1);
        if !self.is_oob(pos) {
            let pos = (pos.0 as usize, pos.1 as usize);
            if self.state[pos] != State::Wall {
                self.state[pos] = default_state;
            }
        }
    }

    fn count(&self, state: State) -> usize {
        let mut cnt = 0;
        for s in self.state.iter() {
            if s == &state {
                cnt += 1;
            }
        }
        cnt
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
        let (height, width) = (grid.dim().0, grid.dim().1);
        match start {
            None => unreachable!("Start not found!"),
            Some(start) => Maze {
                grid,
                start,
                state: Array2::default((height, width)),
                start_tile: None,
            },
        }
    })
}

pub fn compute(input: &str) -> usize {
    let width = &input.lines().next().unwrap().len();
    let (mut maze, _) = maze(*width).easy_parse(input).unwrap();
    maze.init_state();
    maze.floodfill();
    let cnt_unknown = maze.count(State::Unknown);
    let cnt_inside = maze.count(State::Inside);
    if cnt_inside > 0 {
        return cnt_inside;
    }
    cnt_unknown
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
        let input = ".....
.F-7.
.S.|.
.L-J.
.....";
        let width = &input.lines().next().unwrap().len();
        assert_eq!(*width, 5);
        let (mut maze, rest) = maze(*width).easy_parse(input).unwrap();
        assert_eq!(maze.start, (2, 1));
        assert_eq!(
            maze.grid.slice(s![1, ..]),
            ArrayView::from(&[Tile::Ground, Tile::SE, Tile::EW, Tile::SW, Tile::Ground])
        );
        assert!(rest.is_empty());
        let accepted_values = vec![
            (Tile::NS, Direction::N, (1, 1)),
            (Tile::NS, Direction::S, (3, 1)),
        ];
        println!("Loop entry {:?}", &maze.loop_entry());
        assert!(accepted_values.contains(&maze.loop_entry()));
        maze.init_state();
        println!("{:?}", maze.state);
    }

    #[test]
    fn test_init_state() {
        //input 1
        let input = ".............
.S---------7.
.|...F----7|.
.|...|....||.
.|...|....||.
.|...L-7F-J|.
.|.....||..|.
.L-----JL--J.
.............";
        let width = &input.lines().next().unwrap().len();
        let (mut maze, _) = maze(*width).easy_parse(input).unwrap();
        maze.init_state();
        maze.floodfill();
        println!("{:?}", maze.state);
    }
}
