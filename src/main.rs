use crate::tree::playlist::Playlist;
pub mod tree;

fn main() {

    let p: Playlist = Playlist {
        queue: Vec::new(),
        current: 0,
    };

    match p.write() {
        Ok(_) => {
            return;
        }
        Err(err) => {
            println!("Err: {}", err);
            return;
        }
    }
    // yew::Renderer::<app::App>::new().render();
}
