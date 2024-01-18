mod cli;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();
}
