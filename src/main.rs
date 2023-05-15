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
        eprintln!("The provided path is not a directory.");
        process::exit(1);
    }

    // Load the global config
    let global_config_path = format!("{}/.gitzap.json", env::var("HOME").unwrap());
    let mut config = match config::Config::load_from_file(&global_config_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Failed to load global configuration.");
            process::exit(1);
        }
    };

    // Load the repository specific config if it exists and override the global config
    let repo_config_path = format!("{}/.gitzap.json", repo_path);
    if Path::new(&repo_config_path).exists() {
        match config::Config::load_from_file(&repo_config_path) {
            Ok(c) => config = c,
            Err(_) => {
                eprintln!("Failed to load repository specific configuration.");
                process::exit(1);
            }
        }
    }

    let commit_messages = config.commit_messages;
    let commit_timer = config.commit_timer;
    let mut message_index = 0;

    loop {
        // Wait for the timer duration
        sleep(Duration::from_secs(commit_timer as u64)).await;

        // Get the commit message
        let commit_message = &commit_messages[message_index];
        message_index = (message_index + 1) % commit_messages.len();

        // Add, commit and push changes
        if let Err(e) = git::add_commit_push(repo_path, commit_message) {
            eprintln!("Failed to commit and push changes: {}", e);
        }
    }
}
