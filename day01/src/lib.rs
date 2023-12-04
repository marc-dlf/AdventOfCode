use anyhow::Result;
use lazy_static::lazy_static;
use std::path::PathBuf;
use clap::Parser;
use std::collections::{HashMap,HashSet};
use std::cmp;

#[derive(Debug, Clone, Default, Parser)]
pub struct Opts {
    /// Server port.
    #[clap(short, long)]
    pub input_filename: PathBuf,
}

lazy_static!{
    static ref MAP_DIGIT: HashMap<&'static str, char> = [
        ("one",'1'),
        ("two", '2'),
        ("three", '3'),
        ("four",'4'),
        ("five",'5'),
        ("six",'6'),
        ("seven",'7'),
        ("eight",'8'),
        ("nine",'9'),
    ].iter().copied().collect();

    static ref START_CHARS: HashSet<char> = ['o','t','f','s','e','n'].iter().copied().collect();
}

pub struct Calibration{
    pub first:Option<char>,
    pub second:Option<char>
}

impl Calibration{
    fn new() -> Calibration{
        Calibration{first:None,second:None}
    }

    fn update(&mut self,c:char){
        match self.first{
            Some(_) => {self.second=Some(c)},
            None => {(self.first,self.second)=(Some(c),Some(c))}
        }
    }

    fn compute(&self)->usize{
        match (self.first,self.second){
            (Some(first),Some(second)) => vec![first,second].into_iter().collect::<String>().parse::<usize>().unwrap(),
            _ => 0
        }
    }
}

pub fn compute_calibration(line: &str)->Result<usize>{
    let mut cal:Calibration = Calibration::new();
    let n = line.len();

    for (i,c) in line.chars().enumerate(){
        if c.is_digit(10){
            cal.update(c);
            continue
        }
        let remaining = n-i;
        if let Some(&_c) = START_CHARS.get(&c){
            if remaining>=3{
                for offset in 3..(cmp::min(remaining,5)+1){
                    match MAP_DIGIT.get(&line[i..i+offset]){
                        Some(&c) => cal.update(c),
                        None=> {}
                    }
            }
            }
        }
    };

    Ok(cal.compute())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model() {
        let cases = vec![("sevenfourfour99seven8", 78), ("74two24jjsxgvzfqxtwone", 71),("74two24jjsxgvzfqxtwonex", 71)];
        let output = cases.iter().map(|(line,_)| compute_calibration(&line));

        for ((_, expected), calibration) in cases.iter().zip(output) {
            assert_eq!(*expected,calibration.unwrap())
        }
    }
}

