use crate::tree::playlist::Playlist;
pub mod tree;

fn main() {

    let p : Playlist = Playlist::new();

    println!("{}", p.get_selected().unwrap().name);
    
    // yew::Renderer::<app::App>::new().render();
}
