use crate::web::prelude::*;
pub mod tree;
pub mod web;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
