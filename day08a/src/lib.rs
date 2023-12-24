use std::collections::HashMap;
use std::collections::HashSet;
use std::string::String;

use gcd::Gcd;

use combine::{
    many1,
    parser::char::{char, digit, letter, newline, string},
    sep_by, EasyParser, ParseError, Parser, Stream,
};

#[derive(PartialEq, Eq, Debug)]
struct Step {
    left: String,
    right: String,
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Left,
    Right,
}

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

fn node<Input>() -> impl Parser<Input, Output = (String, String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many1(letter()).skip(string(" = (")),
        many1(letter()).skip(string(", ")),
        many1(letter()).skip(char(')')),
    )
        .map(|(entry, left, right): (String, String, String)| (entry, left, right))
}

fn network<Input>() -> impl Parser<Input, Output = (Vec<Direction>, HashMap<String, Step>)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many1(letter()).skip(newline()),
        newline(),
        sep_by(node(), newline()),
    )
        .map(
            |(directions, _, nodes): (Vec<char>, _, Vec<(String, String, String)>)| {
                let directions = directions
                    .into_iter()
                    .map(|c| match c {
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        _ => unreachable!("Direction is not L or R"),
                    })
                    .collect::<Vec<Direction>>();
                let mut network: HashMap<String, Step> = HashMap::new();
                nodes.into_iter().for_each(|(entry, left, right)| {
                    network.insert(entry, Step { left, right });
                });
                (directions, network)
            },
        )
}

pub fn compute(input: &str) -> usize {
    let ((directions, network), _) = network().easy_parse(input).unwrap();
    let mut num_steps: usize = 0;

    //Part 1
    //let (mut current, end) = (network.get("AAA").unwrap(), network.get("ZZZ").unwrap());

    // //works but buggy because doesn't check at each step if okay
    // //actually the input is purposedely a loop which is essenstial to part 2

    // while current != end {
    //     for dir in directions.iter() {
    //         let Step { left, right } = current;
    //         match dir {
    //             Direction::Left => {
    //                 current = network.get(left).unwrap();
    //             }
    //             Direction::Right => {
    //                 current = network.get(right).unwrap();
    //             }
    //         };
    //         num_steps += 1;
    //     }
    // }

    //Part2
    let startpoints: Vec<String> = network
        .keys()
        .filter(|node| node.chars().nth(2).unwrap() == 'A')
        .map(String::from)
        .collect::<Vec<String>>();

    let endpoints: HashSet<&str> = HashSet::from_iter(
        network
            .keys()
            .filter(|node| node.chars().nth(2).unwrap() == 'Z')
            .map(|node| node.as_str()),
    );

    let mut current_points = startpoints.clone();

    println!("{:?}", current_points);
    println!("{:?}", endpoints);
    let mut loop_length: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut endpoints_in_loop: Vec<Vec<usize>> = vec![vec![]; endpoints.len()];

    while loop_length.len() != endpoints.len() {
        for dir in directions.iter() {
            if loop_length.len() == endpoints.len() {
                break;
            } else {
                current_points.iter_mut().for_each(|point| {
                    let Step { left, right } = network.get(point).unwrap();
                    match dir {
                        Direction::Left => {
                            *point = left.to_string();
                        }
                        Direction::Right => {
                            *point = right.to_string();
                        }
                    };
                });

                num_steps += 1;

                for (i, p) in current_points.iter().enumerate() {
                    if (!loop_length.contains_key(&i)) && p.as_str().chars().nth(2).unwrap() == 'Z'
                    {
                        let v = endpoints_in_loop.get_mut(i).unwrap();
                        v.push(num_steps);
                        if v.len() == 3 {
                            loop_length.insert(i, v.clone());
                        }
                    }
                }
            }
        }
    }

    // After observation of the data, it seems that all path loop at the endpoint with a
    // loop size defined by the first element of loop_length for each path

    // Printing of the cycles per starting point
    println!("{:?}", loop_length);

    let loop_length = loop_length
        .into_values()
        .map(|v| *v.first().unwrap())
        .collect::<Vec<usize>>();

    let mut loop_gcd = loop_length
        .first()
        .unwrap()
        .gcd(*loop_length.get(1).unwrap());

    for i in 2..loop_length.len() {
        loop_gcd = loop_gcd.gcd(*loop_length.get(i).unwrap());
    }

    loop_gcd * loop_length.iter().map(|v| v / loop_gcd).product::<usize>()
}

#[cfg(test)]
mod tests {
    use combine::EasyParser;

    use super::*;
    #[test]
    fn test_parse() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let ((directions, network), rest) = network().easy_parse(input).unwrap();
        let expected = network.get("AAA");
        println!("{:?}", network);
        println!("{:?}", rest);

        assert_eq!(
            directions,
            vec![Direction::Left, Direction::Left, Direction::Right]
        );
        assert_eq!(
            expected.unwrap(),
            &Step {
                left: String::from("BBB"),
                right: String::from("BBB"),
            },
        );
        assert!(rest.is_empty());
    }
}
