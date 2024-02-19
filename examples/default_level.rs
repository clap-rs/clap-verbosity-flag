use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
struct DefaultCli {
    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Debug, Parser)]
struct InfoCli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() {
    let error_cli = DefaultCli::parse();

    dbg!(&error_cli.verbose);

    let info_cli = InfoCli::parse();

    dbg!(&info_cli.verbose);
}
