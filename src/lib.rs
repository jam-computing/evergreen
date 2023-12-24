use std::fs;
use serde::{ Serialize, Deserialize };

// I have no idea where this code should go?
// src/tree maybe?

// where config is
// also if we are going to use config files, we should defintely just use one
pub const PATH: &str = "tree.json";    

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {   
    name: String,
    leds: Vec<Colour>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            name: "tree".into(),
            leds: Vec::new(),
        }
    }

    pub fn serialize(&self) -> std::io::Result<()> {
        let s = serde_json::to_string(self).unwrap();  
        fs::write(PATH, s)?;
        Ok(())
    }

    pub fn from_file(path: &str) -> Option<Tree> {
        let file = fs::read_to_string(PATH).unwrap();

        if file == "" {
            return None
        }

        let tree: Tree = serde_json::from_str(file.as_str()).unwrap();
        Some(tree)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    name: String,
    len: usize,
    frames: Vec<Vec<Colour>>,
}

impl Animation {
    fn new() -> Animation {
        Animation {
            name: "default".into(),
            len: 0,
            frames: Vec::new()
        }
    }

    fn from_file(path: &str) -> Option<Animation> {
        let file = fs::read_to_string(PATH).unwrap();
        if file == "" {
            return None
        }
        let animation: Animation = serde_json::from_str(file.as_str()).unwrap();
        Some(animation)
    }
}


