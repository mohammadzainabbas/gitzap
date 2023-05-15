use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub commit_messages: Vec<String>,
    pub commit_timer: i64,  // time in seconds
}

impl Config {
    // pub fn new() -> Self {
    //     Config {
    //         commit_messages: Vec::new(),
    //         commit_timer: 0,
    //     }
    // }

    pub fn load_from_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        println!("Reading configuration from file: {:?}", path);
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;

        Ok(config)
    }
}
