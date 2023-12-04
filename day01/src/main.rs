use anyhow::Result;
use day01::compute_calibration;
use std::fs::File;
use std::io::{BufReader,BufRead};
use clap::Parser;

fn main() -> Result<()>{
    let opts = day01::Opts::parse();
    let f = File::open(&opts.input_filename)?;
    let reader = BufReader::new(f);
    let mut calibration = 0;
    
    for line in reader.lines(){
        calibration+=compute_calibration(&line?)?;
    }
    // outputs result in terminal
    println!("{}",calibration);
    Ok(())
}