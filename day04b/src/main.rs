use clap::Parser;

fn main() {
    let opts = day04b::Opts::parse();
    day04b::compute(opts.input_filename);
}
