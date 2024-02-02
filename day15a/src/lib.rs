use std::string::String;
use std::{collections::HashSet, hash::Hash};

use combine::{
    choice, many1, parser::byte::byte, parser::token::none_of, sep_by1, EasyParser, ParseError,
    Parser, Stream,
};

fn hash(v: Vec<u8>) -> usize {
    let mut current: usize = 0;
    for c in v.iter() {
        current += *c as usize;
        current *= 17;
        current %= 256;
    }
    current
}

fn manual<Input>() -> impl Parser<Input, Output = usize>
where
    Input: Stream<Token = u8>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    sep_by1(
        many1::<Vec<_>, _, _>(none_of(b",".iter().cloned())),
        byte(b','),
    )
    .map(|steps: Vec<Vec<u8>>| {
        let mut total = 0;
        steps
            .into_iter()
            .for_each(|step: Vec<u8>| total += hash(step));
        total
    })
}

pub fn compute(input: &str) -> usize {
    manual().easy_parse(input.as_bytes()).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_parse_islands() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let (output, rest) = manual().easy_parse(input.as_bytes()).unwrap();
        println!("{:?}", output);
        assert!(rest.is_empty());
        // assert_eq!(output.first().unwrap().0, "rn=1".as_bytes());
        // assert_eq!(output.last().unwrap().0, "ot=7".as_bytes());
        assert_eq!(output, 1320);
    }
}
