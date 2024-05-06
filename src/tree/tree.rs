use crate::{db::db_item::DbItem, player::playable::Playable};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tree {
    pub name: String,
    pub frames: Option<String>
}

impl Tree {
    pub fn play(playable: &dyn Playable) {
        playable.play();
    }
    pub fn pause() {

    }
}

impl DbItem for Tree {}
