use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub disallowed_strings: Vec<String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> Config {
        let toml_str =
            fs::read_to_string(file_path).expect(format!("Failed to read {}", file_path).as_str());
        toml::from_str(&toml_str).expect(format!("Failed to deserialize {}", file_path).as_str())
    }
}
