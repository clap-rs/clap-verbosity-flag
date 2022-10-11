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

    tracing_subscriber::fmt()
        .with_max_level(cli.verbose.tracing_level_filter())
        .init();

    log::error!("Engines exploded");
    log::warn!("Engines smoking");
    log::info!("Engines exist");
    log::debug!("Engine temperature is 200 degrees");
    log::trace!("Engine subsection is 300 degrees");
}
