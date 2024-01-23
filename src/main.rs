use clap::Parser;
use gitzap::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{:#?}", args)
}
