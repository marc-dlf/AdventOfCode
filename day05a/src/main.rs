use clap::Parser;

fn main() {
    let opts = day05a::Opts::parse();
    day05a::compute(opts.input_filename);
}
