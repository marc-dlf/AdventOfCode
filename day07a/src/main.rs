use clap::Parser;

fn main() {
    //let opts = day07a::Opts::parse();
    let out = day07a::compute(include_str!("./input.txt"));
    println!("Result is : {}", out);
}
