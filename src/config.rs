use std::fs;

use clap::Parser;
use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    pub disallowed_strings: Vec<String>,
    pub max_token_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            disallowed_strings: Vec::new(),
            max_token_length: 10,
        }
    }
}

#[derive(Parser, Deserialize)]
struct PartialConfig {
    disallowed_strings: Option<Vec<String>>,
    #[arg(short, long)]
    max_token_length: Option<usize>,
}

fn parse_config_file(file_path: &str) -> PartialConfig {
    let toml_str =
        fs::read_to_string(file_path).expect(format!("Failed to read {}", file_path).as_str());
    let toml_config: PartialConfig =
        toml::from_str(&toml_str).expect(format!("Failed to deserialize {}", file_path).as_str());
    toml_config
}

impl Config {
    pub fn load_all(file_path: &str) -> Self {
        // Create default config
        let mut config = Config::default();
        // Parse config.toml
        let toml_config = parse_config_file(file_path);
        // Parse command line args
        let args = PartialConfig::parse();

        if let Some(disallowed_strings) = args.disallowed_strings.or(toml_config.disallowed_strings)
        {
            config.disallowed_strings = disallowed_strings;
        }
        if let Some(max_token_length) = args.max_token_length.or(toml_config.max_token_length) {
            config.max_token_length = max_token_length;
        }

        config
    }
}
