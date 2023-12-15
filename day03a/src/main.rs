use clap::Parser;

fn main() {
    let opts = day03a::Opts::parse();
    let out = day03a::compute(opts.input_filename).unwrap();
    println!("Result is : {}", out);
}
