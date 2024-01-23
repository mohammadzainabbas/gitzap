use crate::GitUtils;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Cli {
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

impl Cli {
    #[allow(dead_code, clippy::wrong_self_convention)]
    fn new(&self) -> Cli {
        todo!()
    }
}

impl GitUtils for Cli {
    #[allow(unused_variables)]
    fn is_git_repo(path: String) -> bool {
        todo!()
    }
}
