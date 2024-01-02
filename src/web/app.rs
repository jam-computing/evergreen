use super::super::tree::playlist::Playlist;
use yew::{ function_component, html, Html, Callback };
use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <button onclick={Callback::from(move |_| {
                Playlist::instance().lock().unwrap().next();
            })}> { "Next" } </button>
        </div>
        <>
            { "Hello" }
        </>
    }
}
