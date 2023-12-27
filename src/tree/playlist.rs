use serde::{ Serialize, Deserialize };
use std::fs;
use super::animation::Animation;

const PLAYLIST_PATH: &str = "playlist.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub queue: Vec<Animation>,
    pub current: usize
}

impl Playlist {
    pub fn new() -> Playlist {
        let mut p: Playlist = Playlist {
            queue:  Vec::new(),
            current:  0
        };
        p.read();
        p
    }

    pub fn read(&mut self) {
        let file = fs::read_to_string(PLAYLIST_PATH).unwrap();
        let v: Vec<Animation> = serde_json::from_str(file.as_str()).unwrap();
        self.queue = v;
    }

    pub fn write(&self) -> std::io::Result<()> {
        let s = serde_json::to_string(&self).unwrap();  
        fs::write(PLAYLIST_PATH, s)?;
        Ok(())
    }

    pub fn play(&mut self) {

    }

    pub fn pause(&mut self) {

    }

    pub fn next(&mut self) {

    }

    pub fn previous(&mut self) {

    }
}
