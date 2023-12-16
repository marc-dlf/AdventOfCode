use clap::Parser;

fn main() {
    let opts = day03b::Opts::parse();
    let out = day03b::compute(opts.input_filename).unwrap();
    println!("Result is : {}", out);
}
