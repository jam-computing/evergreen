use crate::web::prelude::*;
pub mod web;
pub mod tree;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
