use crate::tree::playlist::Playlist;
use crate::web::prelude::*;
pub mod web;
pub mod tree;

fn main() {
    let p = Playlist::new();
    println!("");

    // yew::Renderer::<app::App>::new().render();
}
