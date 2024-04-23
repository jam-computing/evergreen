use crate::player::playable::Playable;

pub struct Tree {

}

impl Tree {
    pub fn play(playable: &dyn Playable) {
        playable.play();
    }
    pub fn pause() {

    }
}
