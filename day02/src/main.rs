use anyhow::Result;
use clap::Parser;
use day02::{self, Observations};
use std::fs::File;
use std::io::{BufRead, BufReader};

// fn main() -> Result<()> {
//     let opts = day02::Opts::parse();
//     let f = File::open(&opts.input_filename)?;
//     let reader = BufReader::new(f);
//     let mut out = 0;
//     let max_vals: HashMap<String, usize> = HashMap::from([
//         ("blue".to_string(), 14),
//         ("red".to_string(), 12),
//         ("green".to_string(), 13),
//     ]);
//     let mut ok_game: bool = true;
//     for line in reader.lines() {
//         if let Ok(obs) = Observations::new(&line.unwrap()) {
//             for (key, val) in obs.max_observed.iter() {
//                 match max_vals.get(key) {
//                     Some(v) => {
//                         if v < val {
//                             ok_game = false;
//                             break;
//                         }
//                     }
//                     None => {
//                         ok_game = false;
//                         break;
//                     }
//                 }
//             }
//             if ok_game {
//                 out += obs.game_id;
//             }
//             ok_game = true;
//         };
//     }
//     // outputs result in terminal
//     println!("{}", out);
//     Ok(())
// }

fn main() -> Result<()> {
    let opts = day02::Opts::parse();
    let f = File::open(&opts.input_filename)?;
    let reader = BufReader::new(f);
    let mut out = 0;
    for line in reader.lines() {
        let mut prod = 1;
        if let Ok(obs) = Observations::new(&line.unwrap()) {
            for (_, val) in obs.max_observed.iter() {
                prod *= val;
            }
        }
        out += prod;
    }
    // outputs result in terminal
    println!("{}", out);
    Ok(())
}
