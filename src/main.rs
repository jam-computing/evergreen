use yew::{virtual_dom::VNode, Properties, function_component, html, Html};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <HelloWorld>
            <span>{"Hello Evergreen!"}</span>
            <h1>{"The Sky"}</h1>
        </HelloWorld>
    }
}

// VNode is just alias for Html
// If function component then practically compiles to <HelloWorld>
#[function_component]
fn HelloWorld(props: &Props) -> VNode {
    // props.on_name_entry.emit(String::from("Matthew"));
    html! { 
        <div>
            { props.children.clone() }
        </div>
    }
}

// Must derive properties in order to use struct as parameter
// The name "children" is really important 
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html
}

