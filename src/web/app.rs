use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <center>
            <h1> { "Hello, World!" } </h1>
        </center>
    }
}
