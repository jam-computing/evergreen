pub trait Playable {
    fn play(&self);
    fn pause(&self);
}

pub struct Animation {
    // List of "frames"
    pub frames: Vec<Vec<(u8, u8, u8)>>,
    // Time per frame
    pub tpf: u32,
    pub playing: bool
}

impl Playable for Animation {
    fn play(&self) {
    }
    fn pause(&self) {
    }
}

