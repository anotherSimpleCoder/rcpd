use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub out_path: String,
}

pub fn load_config() -> Config {
    let mut file = File::open("config.json")
        .expect("ConfigError: config.json not found!");

    let mut config_buffer = String::new();
    let _ = file.read_to_string(&mut config_buffer);
    let config: Config = serde_json::from_str(config_buffer.as_str())
        .expect("ConfigError: Faulty config.json!");

    config
}