use yew::{function_component, html, Html, use_state };

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" } </button>
            <p> { *counter } </p>
        </div>
    }

}

#[function_component(Menu)]
pub fn menu() -> Html {
    html! {
       <> { "Hello, World!" } </> 
    }
}
