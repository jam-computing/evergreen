extern crate rand;

use super::animation::Animation;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::fs;

const PLAYLIST_PATH: &str = "playlist.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Playlist {
    pub queue: Vec<Animation>,
    current: usize,
}

impl Playlist {
    pub fn new() -> Playlist {
        let mut p: Playlist = Playlist {
            queue: Vec::new(),
            current: 0,
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

    pub fn get_selected(&self) -> Option<&Animation> {
        self.queue.get(self.current)
    }

    pub fn next(&mut self) {
        if self.current < self.queue.len() {
            self.current += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn add_many(&mut self, animations: Vec<Animation>) {
        self.queue.extend(animations);
    }

    pub fn add(&mut self, animation: Animation) {
        self.queue.push(animation);
    }

    pub fn remove_at(&mut self, index: usize) {
        self.queue.remove(index);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.queue.shuffle(&mut rng);
    }
}
