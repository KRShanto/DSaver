use crate::*;

/// Manage the state of an input element.
///
/// You can use this hook to update the value of an input element and get notified when the value changes.
///
/// # Returns
///
/// It returns a tuple and values are:
///
/// 0.`value` - The current value of the input element.
///
/// 1.`callback` - A callback that can be used to update the value of the input element.
///
/// You can use that callback with `onkeyup`, `onchange`, `onkeydown` etc. events.
///
/// You can also change the `value` to update the input element.
///
/// The parameter `default` is the initial value of the input element.
///
/// # Example
///
/// ```
/// use yew::prelude::*;
/// use weblog::console_log;
/// use dsaver_frontend::{use_input, UseInputOptions};
///
/// #[function_component(Name)]
/// fn name() -> Html {
///     let (name, onkeyup) = use_input("", &UseInputOptions::default());
///
///     {
///         let name = name.clone();
///         use_effect_with_deps(move |name| {
///             console_log!("Your name is {}", &**name);
///
///             || ()
///         }, name); // When `name` changes, do something
///     }
///     
///     html! {
///         <>
///             <form>
///                 <label for="name">{"Your name"}</label>
///                 <input
///                     type="text"
///                     class="name"
///                     name="name"
///                     value={(*name).clone()}
///                     {onkeyup}
///                 />
///             </form>
///         </>
///     }
/// }
/// ```
pub fn use_input(
    default: &str,
    options: &UseInputOptions,
) -> (UseStateHandle<String>, Callback<KeyboardEvent>) {
    // input permission
    let permission = options.permission.clone();

    // state of the input
    let input_state = use_state(|| default.to_string());

    // Callback to update the input state
    let when_changes = Callback::from({
        let input_state = input_state.clone();

        move |event: KeyboardEvent| {
            // getting the target from the event
            let event = event.target().unwrap();
            // converting the event
            let value = event.dyn_into::<HtmlInputElement>().unwrap().value();

            // if the input has write and read permission then update the state
            if permission == InputPermission::WriteAndRead {
                input_state.set(value);
            }
        }
    });

    (input_state, when_changes)
}

/// Opitons for the [`use_input()`] hook.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UseInputOptions {
    /// The type of the input element.
    pub input_type: InputType,
    /// The permission of the input element.
    pub permission: InputPermission,
}

impl UseInputOptions {
    /// Set the `input_type` of the input element.
    pub fn input_type(itype: InputType) -> Self {
        Self {
            input_type: itype,
            ..Default::default()
        }
    }

    /// Set the `permission` of the input element.
    pub fn permission(perm: InputPermission) -> Self {
        Self {
            permission: perm,
            ..Default::default()
        }
    }
}

/// Type of the input
///
/// At present, only `text` and `number` are supported.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
// TODO: only take those characters (from input) who are allowed to
pub enum InputType {
    /// `text` input
    #[default]
    Text,
    /// `number` input
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

/// Permission of the input element.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum InputPermission {
    /// The user can type, read, and change the value of the input element.
    #[default]
    WriteAndRead,
    /// The user cannot type, but can read the value of the input element.
    ReadOnly,
    /// The user neither can type nor read the value of the input element.
    Disabled,
}
