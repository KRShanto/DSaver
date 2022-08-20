use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Hello World!"}</h1>
        </div>
    }
}
