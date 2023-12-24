use std::fs;
use serde::{ Serialize, Deserialize };
// I have no idea where this code should go?
// src/tree maybe?


pub const PATH: &str = "tree.json";    

pub fn get_tree() -> Option<Tree> {
    let file = fs::read_to_string(PATH).unwrap();
    if file == "" {
        return None
    }
    let out: Tree = serde_json::from_str(file.as_str()).unwrap();
    Some(out)
}

pub fn serialize_tree(tree: &Tree) -> std::io::Result<()> {
    let s = serde_json::to_string(tree).unwrap();  
    fs::write(PATH, s)?;
    Ok(())
}

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Colour {
    red: u8,
    green: u8,
    blue: u8
}
