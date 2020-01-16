use clap_verbosity_flag::Verbosity;
use structopt::StructOpt;

/// Foo
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() {
    Cli::from_args();
}
