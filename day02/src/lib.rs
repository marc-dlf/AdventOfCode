use anyhow::{anyhow, Result};
use clap::Parser;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Parser)]
pub struct Opts {
    #[clap(short, long)]
    pub input_filename: PathBuf,
}

#[derive(Debug)]
pub struct Observations {
    pub max_observed: HashMap<String, usize>,
    pub game_id: usize,
}

impl Observations {
    pub fn new(line: &str) -> Result<Observations> {
        let mut observed: HashMap<String, usize> = HashMap::new();
        //regex to separate game id from the rest
        let re1 = Regex::new(r"([0-9]+):(.+)").unwrap();
        //regex to separate games from each other
        let re2 = Regex::new(r"[^;]+").unwrap();
        //regex to obtain results of drawings
        let re3 = Regex::new(r"([0-9]+) ([a-zA-Z]+)").unwrap();

        let (_, [id, games]) = re1.captures(line).unwrap().extract();
        let id: usize = id.parse().map_err(|e| anyhow!("Invalid {}: {e}", id))?;

        for game in re2.find_iter(games) {
            for (_, [number, color]) in re3.captures_iter(game.as_str()).map(|c| c.extract()) {
                {
                    let number: usize = number.parse().unwrap();
                    observed
                        .entry(color.to_string())
                        .and_modify(|e| *e = cmp::max(*e, number))
                        .or_insert(number);
                }
            }
        }
        Ok(Observations {
            max_observed: observed,
            game_id: id,
        })
    }
}
