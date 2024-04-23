use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::log::logger::warn;

use super::config::Config;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

#[allow(dead_code)]
impl Config for ServerConfig {
    fn read_from_file() -> Option<ServerConfig> {
        let mut file: File;

        match File::open("config.json") {
            Ok(v) => file = v,
            Err(e) => {
                warn("Could not open config.json. It may be used by another process.");
                return None;
            }
        };

        let mut json = String::new();

        if let Err(_) = file.read_to_string(&mut json) {
            warn("Could not read config.json. It may be being used in another process.");
            return None;
        }

        let conf: ServerConfig;
        match serde_json::from_str(&json) {
            Ok(v) => conf = v,
            Err(e) => {
                warn("Could not deserialize configuration file");
                return None;
            }
        }
        Some(conf)
    }
}
