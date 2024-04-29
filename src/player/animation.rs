#![allow(dead_code)]

use super::playable::Playable;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Animation {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Artist")]
    pub artist: String,
    #[serde(rename = "Frames")]
    pub frames: Option<String>,
    #[serde(rename = "Tick_Rate")]
    pub tick_rate: u32,
    #[serde(rename = "collectionId")]
    pub collection_id: String,
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    #[serde(rename = "created")]
    pub created: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "updated")]
    pub updated: String,
}

impl Animation {
    pub fn from(json: &str) -> serde_json::Result<Vec<Self>> {
        let animations: Vec<Animation> = serde_json::from_str(json)?;
        Ok(animations)
    }

    pub fn new() -> Animation {
        Animation {
            title: "".into(),
            artist: "".into(),
            collection_id: "".into(),
            collection_name: "".into(),
            created: "".into(),
            frames: None,
            id: "".into(),
            tick_rate: 0,
            updated: "".into()
        }
    }
}

impl Playable for Animation {
    fn play(&self) {
        println!("Playing Animation: {}", self.title);
    }

    fn pause(&self) {
        println!("Paused Animation: {}", self.title);
    }
}
