use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Link {
    url: String,       // manually
    title: String,     // automatically
    tags: Vec<String>, // manually  // filterable --- keep|remove
    prirority: char,   // manually  // filterable --- A, B, C...|...C, B, A
    browser: String,   // manually  // filterable --- keep|remove
    complete: bool,    // manually  // filterable --- ture-false|false-true
    date: String,      // automatically // filterable --- latest|oldest
}

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
