use super::playlist::Playlist;

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
