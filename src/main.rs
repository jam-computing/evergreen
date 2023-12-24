use crate::web::prelude::*;
pub mod web;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
