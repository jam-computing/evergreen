use super::playlist::Playlist;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// The animator class contains all the data
// associated with the thread that does the animation
//
// 'playlist_arc' -> thread safe pointer to the playlist
// 'thread_join_handle' -> join join_handle of the created thread
// 'frame_rate' -> thread safe usize which determines the time between frames in ms
#[derive(Debug)]
pub struct Animator {
    pub playlist_arc: Arc<Mutex<Playlist>>,
    pub thread_join_handle: Option<JoinHandle<()>>,
    pub frame_delay: Arc<Mutex<u64>>,
}

impl Animator {
    // This returns a new animation struct and starts the associated thread
    pub fn new(playlist_arc: Arc<Mutex<Playlist>>) -> Animator {
        let mut a = Animator {
            playlist_arc: playlist_arc.clone(),
            thread_join_handle: None,
            frame_delay: Arc::new(Mutex::new(15)),
        };

        // Variables the thread needs to communicate
        let frame_delay = a.frame_delay.clone(); // Approx 60 fps
        let thread_playlist_arc = playlist_arc.clone();

        let join_handle = thread::spawn(move || {
            let current_playlist = thread_playlist_arc;
            let current_frame_delay = frame_delay;

            loop {
                thread::sleep(Duration::from_millis(*current_frame_delay.lock().unwrap()));

                // Display the animation on the tree here
            }
        });

        a.thread_join_handle = Some(join_handle);
        a
    }
}
