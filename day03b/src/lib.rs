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

#[derive(Debug)]
pub struct Number {
    pub start: usize,
    pub end: usize,
    pub val: usize,
}

pub fn compute_gear(row: usize, col: usize, numbers: &HashMap<usize, Vec<Number>>) -> usize {
    let low = match row {
        0 => 0,
        v => v - 1,
    };
    let mut gear_linked = vec![];

    for r in low..row + 2 {
        if let Some(v) = numbers.get(&r) {
            for num in v.iter() {
                if num.start <= col + 1 && col <= num.end {
                    gear_linked.push(num.val);
                };
            }
        }
    }
    match &gear_linked[..] {
        [v1, v2] => v1 * v2,
        _ => 0,
    }
}

pub fn compute(filename: PathBuf) -> Result<usize> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    let re_sym = Regex::new(r"(\*)").unwrap();
    let re_num = Regex::new(r"([0-9]+)").unwrap();
    let mut numbers: HashMap<usize, Vec<Number>> = HashMap::new();
    let mut total = 0;

    let lines: Vec<_> = reader.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        if let Ok(l) = line.as_ref() {
            let row = re_num
                .find_iter(l)
                .map(|m| Number {
                    start: m.start(),
                    end: m.end(),
                    val: m.as_str().parse::<usize>().unwrap(),
                })
                .collect::<Vec<Number>>();
            if !row.is_empty() {
                numbers.insert(i, row);
            }
        }
    }

    for (j, line) in lines.iter().enumerate() {
        if let Ok(l) = line.as_ref() {
            for m in re_sym.find_iter(l) {
                total += compute_gear(j, m.start(), &numbers);
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
        assert_eq!(out.unwrap(), 467835);
    }
}
