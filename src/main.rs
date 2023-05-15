use tokio::time::{sleep, Duration};
use std::env;
use std::path::Path;
use std::process;

mod config;
mod git;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a directory path.");
        process::exit(1);
    }

    let repo_path = &args[1];
    if !Path::new(repo_path).is_dir() {
        eprintln!("The provided path is
