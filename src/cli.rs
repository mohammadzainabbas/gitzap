//! `cli` module contains necessary _functions_ and _structs_ to define args from that are usable with the `#[derive(...)]`
//! macros in `clap_derive`.

use crate::GitUtils;
use clap::Parser;

/// Parse command-line arguments into `Self`.
///
/// The primary one-stop-shop trait used to create an instance of a `clap`
/// [`Command`], conduct the parsing, and turn the resulting [`ArgMatches`] back
/// into concrete instance of the user struct.
///
/// This trait is primarily a convenience on top of [`FromArgMatches`] +
/// [`CommandFactory`] which uses those two underlying traits to build the two
/// fundamental functions `parse` which uses the `std::env::args_os` iterator,
/// and `parse_from` which allows the consumer to supply the iterator (along
/// with fallible options for each).
///
/// See also [`Subcommand`] and [`Args`].
///
/// **NOTE:** Deriving requires the `derive` feature flag
#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Args {
    #[clap(
        short = 'c',
        long = "config",
        help = "Configuration file path",
        default_value = "config.toml",
        display_order = 1
    )]
    config: String,

    #[clap(
        short = 'p',
        long = "path",
        help = "Path for git repo",
        default_value = ".",
        display_order = 2
    )]
    path: String,
}

impl Args {
    #[allow(dead_code, clippy::wrong_self_convention)]
    fn new(&self) -> Args {
        todo!()
    }
}

impl GitUtils for Args {
    #[allow(unused_variables)]
    fn is_git_repo(path: String) -> bool {
        todo!()
    }
}
