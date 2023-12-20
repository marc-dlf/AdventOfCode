use clap::Parser;

fn main() {
    let opts = day04a::Opts::parse();
    day04a::compute(opts.input_filename);
}
