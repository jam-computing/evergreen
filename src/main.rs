use crate::tree::playlist::Playlist;
pub mod web;
pub mod tree;

fn main() {
    println!("Name; {}", Playlist::instance().lock().unwrap().get_selected().unwrap().name);
    Playlist::instance().lock().unwrap().next();
    println!("Name; {}", Playlist::instance().lock().unwrap().get_selected().unwrap().name);
    Playlist::instance().lock().unwrap().next();
    println!("Name; {}", Playlist::instance().lock().unwrap().get_selected().unwrap().name);
    // yew::Renderer::<app::App>::new().render();
}
