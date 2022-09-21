use crate::*;

// TODO:give example
pub fn use_input(
    default: String,
    options: &UseInputOptions,
) -> (UseStateHandle<String>, Callback<KeyboardEvent>) {
    let permission = options.permission.clone();

    let input_state = use_state(|| default);

    let onkeyup = Callback::from({
        let input_state = input_state.clone();

        move |event: KeyboardEvent| {
            let event = event.target().unwrap();
            let value = event.dyn_into::<HtmlInputElement>().unwrap().value();

            if permission == InputPermission::WriteAndRead {
                input_state.set(value);
            } else {
                // set the old value
                input_state.set((*input_state).clone());
            }
        }
    });

    (input_state, onkeyup)
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UseInputOptions {
    pub input_type: InputType,
    pub permission: InputPermission,
}

impl UseInputOptions {
    pub fn input_type(itype: InputType) -> Self {
        Self {
            input_type: itype,
            ..Default::default()
        }
    }

    pub fn permission(perm: InputPermission) -> Self {
        Self {
            permission: perm,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
// TODO: only take those characters (from input) who are allowed to
pub enum InputType {
    #[default]
    Text,
    Number,
    // Char,
    // Password,
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Text => f.write_str("text"),
            InputType::Number => f.write_str("number"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum InputPermission {
    #[default]
    WriteAndRead,
    ReadOnly,
    Disabled,
}
