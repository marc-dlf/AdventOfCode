use ndarray::{Array2, Axis};
use std::string::String;

//use ::polyfit_rs::polyfit_rs::polyfit;
use combine::{
    many1,
    parser::char::{char, digit, newline, space},
    sep_by1, EasyParser, ParseError, Parser, Stream,
};

fn _usize<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (many1(digit())).map(|number: String| number.parse::<usize>().unwrap())
}

fn hot_springs<Input>() -> impl Parser<Input, Output = Vec<HotSpring>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (sep_by1(
        (
            many1::<String, _, _>(char('?').or(char('#').or(char('.')))).skip(space()),
            sep_by1::<Vec<usize>, _, _, _>(_usize(), char(',')),
        ),
        newline(),
    ),)
        .map(|(springs,): (Vec<(String, Vec<usize>)>,)| {
            springs
                .into_iter()
                .map(|(springs, damaged)| HotSpring { springs, damaged })
                .collect::<Vec<HotSpring>>()
        })
}

#[derive(Debug)]
pub struct HotSpring {
    springs: String,
    damaged: Vec<usize>,
}

pub fn could_fit(springs: &str, start_idx: usize, n_damaged: usize) -> bool {
    let end_idx = start_idx + n_damaged;
    if end_idx > springs.len() {
        return false;
    }
    if springs[start_idx..end_idx].contains('.') {
        false
    } else {
        !((end_idx < springs.len()) & (springs.chars().nth(end_idx).is_some_and(|c| c == '#')))
    }
}

pub fn decode(spring: &HotSpring, repeat: usize) -> usize {
    let remaining_damaged = spring.damaged.repeat(repeat);
    let mut springs = spring.springs.clone();
    for _ in 0..repeat - 1 {
        springs.push('?');
        springs.push_str(&spring.springs);
    }

    let mut memo: Array2<usize> = Array2::default((springs.len() + 1, remaining_damaged.len()));

    //init first
    let mut seen_hashtag = false;
    for i in 0..memo.dim().0 {
        if (could_fit(&springs, i, remaining_damaged[0])) & !(seen_hashtag) {
            memo[(i + remaining_damaged[0] - 1, 0)] = 1;
        }
        if springs.chars().nth(i).is_some_and(|c| c == '#') {
            seen_hashtag = true;
        }
    }

    for j in 1..memo.dim().1 {
        for i in 0..memo.dim().0 {
            seen_hashtag = false;

            if memo[(i, j - 1)] > 0 {
                for k in i + 2..memo.dim().0 {
                    if seen_hashtag {
                        break;
                    }
                    if springs.chars().nth(k).is_some_and(|c| c == '#') {
                        seen_hashtag = true;
                    }
                    if could_fit(&springs, k, remaining_damaged[j]) {
                        memo[(k + remaining_damaged[j] - 1, j)] += memo[(i, j - 1)]
                    }
                }
            }
        }
    }
    //filtering cases where there is an # after finding all damaged
    for i in 0..memo.dim().0 - 1 {
        if springs[i + 1..].contains('#') {
            memo[(i, remaining_damaged.len() - 1)] = 0;
        }
    }
    memo.sum_axis(Axis(0))[remaining_damaged.len() - 1]
}

pub fn compute(input: &str) -> usize {
    let (hot_springs, _) = hot_springs().easy_parse(input).unwrap();
    let mut total = 0;
    hot_springs.into_iter().for_each(|s| total += decode(&s, 5));
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        let (output, rest) = hot_springs().easy_parse(input).unwrap();
        assert!(rest.is_empty());
        let first_hotspring = output.first().unwrap();
        assert_eq!(first_hotspring.springs, String::from("???.###"));
        assert_eq!(first_hotspring.damaged, vec![1, 1, 3]);
    }

    #[test]
    fn test_could_fit() {
        let springs = "???..#?#?.##";

        assert!(could_fit(springs, 0, 1));
        assert!(could_fit(springs, 0, 2));
        assert!(could_fit(springs, 0, 3));
        assert!(!could_fit(springs, 0, 4));
        assert!(could_fit(springs, 1, 1));
        assert!(!could_fit(springs, 3, 1));
        assert!(could_fit(springs, 5, 1));
        assert!(!could_fit(springs, 5, 2));
        assert!(could_fit(springs, 11, 1));
        assert!(!could_fit(springs, 11, 2));
        assert!(!could_fit(springs, 12, 1)); //start oob

        let springs_2 = "???.###";
        assert!(!could_fit(springs_2, 5, 3));
        assert!(could_fit(springs_2, 4, 3));

        let springs_3 = "?###???????? 3,2,1";
        assert!(!could_fit(springs_3, 0, 3));
    }

    #[test]
    fn test_decode() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
?####???.???#? 6,1,2";

        let (output, _) = hot_springs().easy_parse(input).unwrap();
        let first_hotspring = output.first().unwrap();
        assert_eq!(decode(first_hotspring, 1), 1);
        let last_hotspring = output.last().unwrap();
        assert_eq!(decode(last_hotspring, 1), 8);
    }
}

//  dp[i,j] n*m -> n => dp[0,j] => number of ways to decode first pattern that end
//  at index j
//  dp[1,j] => number of ways to decode first two patterns that end at idex j
//  dp[1,j] = dp[0,j-k] + decode(&str[j-k,j]) -> if dp[i]
//  dp[]

//  dp[i,j] = sum(k) dp[i-1,j-k] + decode(&str[j-k,j]) and if dp[i-1] == 0 directly put 0
//  and when decode reaches a terminal state, stop
