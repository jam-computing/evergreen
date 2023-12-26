use serde::{Serialize, Deserialize};
// NO CLUE WHAT TO CALL THIS FILE

pub const PATH: &str = "tree.json";    

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

