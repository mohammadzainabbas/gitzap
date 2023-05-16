use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::process::Command;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitInfo {
    pub user_name: String,
    pub user_email: String,
    pub remote_name: String,
    pub branch_name: String,
}

impl GitInfo {
    // pub fn new() -> Self {
    //     GitInfo {
    //         user_name: String::new(),
    //         user_email: String::new(),
    //         remote_name: String::new(),
    //         branch_name: String::new(),
    //     }
    // }

    pub fn load_from_temp() -> Result<GitInfo, Box<dyn std::error::Error>> {
        println!("Reading Git information from temporary file: gitzap_temp.json");
        let file = File::open("gitzap_temp.json")?;
        let reader = BufReader::new(file);
        let git_info = serde_json::from_reader(reader)?;

        Ok(git_info)
    }

    pub fn write_to_temp(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Writing Git information to temporary file: gitzap_temp.json");
        let file = File::create("gitzap_temp.json")?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;

        Ok(())
    }
}

pub fn get_git_info() -> GitInfo {
    let user_name = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .output()
        .expect("Failed to fetch git user name")
        .stdout;

    let user_email = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .output()
        .expect("Failed to fetch git user email")
        .stdout;

    let remote_name = Command::new("git")
        .arg("remote")
        .output()
        .expect("Failed to fetch git remote name")
        .stdout;

    let branch_name = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("Failed to fetch git branch name")
        .stdout;

    GitInfo {
        user_name: String::from_utf8(user_name).unwrap().trim().to_string(),
        user_email: String::from_utf8(user_email).unwrap().trim().to_string(),
        remote_name: String::from_utf8(remote_name).unwrap().trim().to_string(),
        branch_name: String::from_utf8(branch_name).unwrap().trim().to_string(),
    }
}

pub fn get_git_token() -> Option<String> {
    let home_dir = dirs::home_dir().expect("Could not get home directory");
    let global_config_path = home_dir.join(".gitzap.json");

    if global_config_path.exists() {
        match Config::load_from_file(global_config_path.to_str().expect("Could not read global config file")) {
            Ok(config) => config.git_token,
            Err(err) => {
                eprintln!("Failed to read global config file: {}", err);
                None
            }
        }
    } else {
        None
    }
}
