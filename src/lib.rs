// The app should go in here

use yew::{ function_component, Html, html };

pub fn render() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! {
        <p>{"Hello, World!"}</p>
    }
}
