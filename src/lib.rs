use std::fs;

// Using serde for json serialization 
use serde::{ Serialize, Deserialize };

// I have no idea where this code should go?
// src/tree maybe?

// where config is
// also if we are going to use config files, we should defintely just use one
pub const PATH: &str = "tree.json";    

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {   
    pub name: String,
    pub locations: Vec<Point>
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            name: "tree".into(),
            locations: Vec::new()
        }
    }

    pub fn serialize(&self) -> std::io::Result<()> {
        let s = serde_json::to_string(self).unwrap();  
        fs::write(PATH, s)?;
        Ok(())
    }

    pub fn from_file(path: Option<&str>) -> Option<Tree> {
        let file = fs::read_to_string(path.unwrap_or(PATH)).unwrap();

        if file == "" {
            return None
        }

        let tree: Tree = serde_json::from_str(file.as_str()).unwrap();
        Some(tree)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize
}


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
        let file = fs::read_to_string(path).unwrap();
        if file == "" {
            return None
        }
        let animation: Animation = serde_json::from_str(file.as_str()).unwrap();
        Some(animation)
    }
}
