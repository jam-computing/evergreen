use crate::tree::playlist::Playlist;
pub mod web;
use crate::web::prelude::*;
pub mod tree;
pub mod web;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
