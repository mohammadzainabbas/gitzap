

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    println!("{:#?}", args)
}
