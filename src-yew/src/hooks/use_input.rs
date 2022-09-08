use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub fn use_input(default: &str) -> (UseStateHandle<String>, Callback<KeyboardEvent>) {
    let input_state = use_state(|| default.to_string());

    let onkeyup = Callback::from({
        let input_state = input_state.clone();

        move |event: KeyboardEvent| {
            let event = event.target().unwrap();
            let value = event.dyn_into::<HtmlInputElement>().unwrap().value();

            input_state.set(value);
        }
    });

    (input_state, onkeyup)
}
