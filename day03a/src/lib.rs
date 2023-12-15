use clap::Parser;
use regex::Regex;
use std::collections::HashMap;

use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Parser)]
pub struct Opts {
    #[clap(short, long)]
    pub input_filename: PathBuf,
}

pub struct Number {
    pub start: usize,
    pub end: usize,
    pub val: usize,
}

pub fn valid(row: usize, number: &Number, symbols: &HashMap<usize, Vec<usize>>) -> bool {
    let low = match row {
        0 => 0,
        v => v - 1,
    };
    for r in low..row + 2 {
        if let Some(v) = symbols.get(&r) {
            for s_idx in v.iter() {
                if number.start <= *s_idx + 1 && *s_idx <= number.end {
                    return true;
                };
            }
        }
    }
    false
}

pub fn compute(filename: PathBuf) -> Result<usize> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    //let re_sym = Regex::new(r"([^0-9\.])").unwrap();
    let re_sym = Regex::new(r"(\*)").unwrap();
    let re_num = Regex::new(r"([0-9]+)").unwrap();
    let mut symbols: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut total = 0;

    let lines: Vec<_> = reader.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if let Ok(l) = line.as_ref() {
            let row = re_sym
                .find_iter(l)
                .map(|m| m.start())
                .collect::<Vec<usize>>();
            if !row.is_empty() {
                symbols.insert(i, row);
            }
        };
    }

    for (j, line) in lines.iter().enumerate() {
        if let Ok(l) = line.as_ref() {
            for m in re_num.find_iter(l) {
                let num = Number {
                    start: m.start(),
                    end: m.end(),
                    val: m.as_str().parse::<usize>().unwrap(),
                };
                if valid(j, &num, &symbols) {
                    total += num.val;
                }
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_regex() {
        let re_num = Regex::new(r"([0-9]+)").unwrap();
        let out = re_num
            .find_iter("...4323...43")
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();
        assert_eq!(out, vec!["4323", "43"]);
    }

    #[test]
    fn test_compute() {
        let out = compute(PathBuf::from("src/test.txt"));
        assert_eq!(out.unwrap(), 4361);
    }
}
