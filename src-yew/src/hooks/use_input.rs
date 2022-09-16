use crate::*;

// TODO:give example
pub fn use_input(
    default: &str,
    options: &UseInputOptions,
) -> (UseStateHandle<String>, Callback<KeyboardEvent>) {
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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UseInputOptions {
    pub disabled: bool,
    pub input_type: InputType,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Number,
    // Char
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => f.write_str("text"),
            InputType::Number => f.write_str("number"),
        }
    }
}
