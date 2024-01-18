use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
struct Cli {
    #[clap(
        short = 'c',
        long = "config",
        help = "Configuration file path",
        default_value = "config.toml",
        display_order = 1
    )]
    config: String,
}
