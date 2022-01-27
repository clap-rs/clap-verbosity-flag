use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Foo
#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
}

fn main() {
    let cli = Cli::parse();

    if let Some(level) = cli.verbose.log_level() {
        env_logger::Builder::new()
            .filter_level(level.to_level_filter())
            .init();
    }
}
