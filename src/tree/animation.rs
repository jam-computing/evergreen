use crate::tree::misc::Colour;
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    name: String,
    len: usize,
    frames: Vec<Vec<Colour>>,
}

impl Animation {
    pub fn new() -> Animation {
        Animation {
            name: "default".into(),
            len: 0,
            frames: Vec::new()
        }
    }

    pub fn from_file(path: &str) -> Option<Animation> {
        let file = std::fs::read_to_string(path).unwrap();
        if file == "" {
            return None
        }
        Some(serde_json::from_str(file.as_str()).unwrap())
    }
}
