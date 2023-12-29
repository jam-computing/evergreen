use super::animation::Animation;
use super::playlist::Playlist;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Animator {
    pub playlist: Playlist,
}

impl Animator {
    pub fn new() -> Animator {
        Animator {
            playlist: Playlist::new(),
        }
    }

    fn animate() {}
}

enum AnimatorState {
    Playing,
    Paused,
}
