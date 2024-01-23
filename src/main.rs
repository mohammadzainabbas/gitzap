use clap::Parser;
use gitzap::Cli;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    println!("{:#?}", args)
}
