
use clap::Parser;
use clap_verbosity_flag::Verbosity;

/// Foo
#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,
}

#[cfg(feature = "tracing")]
fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(cli.verbose.log_level_filter())
        .init();

    tracing::error!("Engines exploded");
    tracing::warn!("Engines smoking");
    tracing::info!("Engines exist");
    tracing::debug!("Engine temperature is 200 degrees");
    tracing::trace!("Engine subsection is 300 degrees");
}

#[cfg(not(feature = "tracing"))]
fn main() {}