#![allow(dead_code)]

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Animation {
    pub title: String,
    pub artist: String,
    pub tick_rate: u16,
    pub frames: Vec<Vec<(u8, u8, u8)>>,
}

impl Animation {
    pub fn from(frames: Vec<Vec<(u8, u8, u8)>>) -> Self {
        Self {
            title: "Super Cool Animation".into(),
            artist: "Super Cool Artist".into(),
            tick_rate: 50 as u16,
            frames
        }
    }
}
