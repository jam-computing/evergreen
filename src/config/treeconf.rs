use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::log::logger::warn;

use super::config::Config;

#[derive(Serialize, Deserialize)]
pub struct TreeConfig {
    pub name: String,
    pub led_locations: (u8, u8, u8) // Normalised 0 and 1
}

impl Config for TreeConfig {
    fn read_from_file() -> Option<TreeConfig> {
        let mut file: File;
        match File::open("treeconfig.json") {
            Ok(v) => file = v,
            Err(_) => {
                warn("Could not read treeconfig.json. It may be being used in another process.");
                return None;
            }
        }

        let mut json = String::new();

        if let Err(e) = file.read_to_string(&mut json) {
            warn("Could not read treeconfig.json. It may be being used in another process.");
            return None;
        }

        let conf: TreeConfig;

        match serde_json::from_str(&json) {
            Ok(v) => conf = v,
            Err(_) => {
                warn("Could not deserialize treeconfig.json. Please check that there is valid json in the file");
                return None;
            }
        }

        Some(conf)
    }
}
