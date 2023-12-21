use clap::Parser;

fn main() {
    let opts = day05b::Opts::parse();
    day05b::compute(opts.input_filename);
}
