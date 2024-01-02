use super::animation::Animation;
use super::playlist::Playlist;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Th

#[derive(Debug)]
pub struct Animator {
    pub playlist_arc: Arc<Mutex<Playlist>>,
}

impl Animator {
    // This returns a new animation struct and starts the associated thread
    pub fn new(playlist_arc: Arc<Mutex<Playlist>>) -> Animator {
        let a = Animator {
            playlist_arc: playlist_arc,
        };
        let join_handle = thread::spawn(|| {
            loop {
                // Animate stuff
            }
        });
        a
    }

    fn animate() {
        Animator::new
    }
}

enum AnimatorState {
    Playing,
    Paused,
}
