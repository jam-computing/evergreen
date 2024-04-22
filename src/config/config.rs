use std::{io::prelude::*, fs::File, io::Read};

use serde::{Serialize, Deserialize};
use serde_json::{to_string, Result};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

#[allow(dead_code)]
impl Config {
    pub fn write_to_file() -> std::io::Result<()> {
        let file = File::create("config.json");
        let config = Config {
            ip: "localhost".to_string(),
            port: 8080
        };

        let json = to_string(&config).unwrap();
        let _ = file?.write_all(json.as_bytes());

        Ok(())
    }
    pub fn get_from_file() -> Result<Config> {
        let mut file = File::open("config.json").expect("Could not read the json file");
        let mut json = String::new();
        file.read_to_string(&mut json)
            .expect("Could not read the json file");

        let conf: Config = serde_json::from_str(&json)?;
        Ok(conf)
    }
}
