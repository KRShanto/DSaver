use crate::*;

mod checkbox;
mod input_div;
mod input_tags;
mod input_wrapper;
mod label;

pub use checkbox::*;
pub use input_div::*;
pub use input_tags::*;
pub use input_wrapper::*;
pub use label::*;

/// Struct that represents the id of the input.
///
/// Making it a struct instead of a string, so that it can be used as a context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(self) struct InputId(String);

/// Props of the [`Input`] component.
#[derive(Properties, PartialEq, Clone, Debug)]
pub struct InputProps {
    /// The state of the value of the input.
    ///
    /// This is a state that is used to store the value of the input.
    ///
    /// If you need to update the input value by another way (not by the `<input>` tag), then you can use this state.
    ///
    /// It is optional. If you don't need to update the input value from outside, then you don't need to use this.
    ///
    /// Note that if you this state, and you also pass a value to `init_value`, then the `init_value` will be ignored.
    #[prop_or_default]
    pub value_state: Option<UseStateHandle<String>>,
    /// Options for the input
    #[prop_or_default]
    pub options: UseInputOptions,
    /// Initial value of the input (optional)
    #[prop_or_default]
    pub init_value: String,
    /// Will the input be focused when the component is mounted?
    ///
    /// If true, then when the component is rendered, the user's cursor will be focused on the input.
    ///
    /// Be careful when using this prop. If you use this to multiple inputs, then unexpected behavior can happen.
    #[prop_or_default]
    pub init_focus: bool,

    /// Paste from clipboard option
    ///
    /// If true, then the user can paste from clipboard
    #[prop_or_default]
    pub paste: bool,
}
/// Input component.
///
/// It makes a beautiful animated input.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`InputProps`]
#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        value_state,
        options,
        init_value,
        init_focus,
        paste,
    } = (*props).clone();

    // Getting the id from the parent component
    let id = format!("input-{}", use_context::<InputId>().unwrap().0);

    let UseInputOptions {
        input_type,
        permission,
    } = options.clone();

    // Getting the initial value
    // If the user passed a `value_state`, and if the value is not empty, then we will use the value from the state.
    // Otherwise, if the user passed an `init_value`, and if the value is not empty, then we will use the value from the `init_value`.
    // Otherwise, we will use an empty string.
    let init_value_input = if let Some(value_state) = value_state.clone() {
        let value = value_state.to_string();

        if !value.is_empty() {
            value
        } else {
            String::new()
        }
    } else if !init_value.is_empty() {
        init_value.clone()
    } else {
        String::new()
    };

    // Getting the input's value from the `onkeyup` event
    let (value, onkeyup) = use_input(&init_value_input, &options);

    {
        // If `value_state` is not empty, or if `init_value` is not empty, then up the label to the top (animation)
        let value_state = value_state.clone();
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(value_state) = value_state {
                    if !value_state.to_string().is_empty() {
                        // If `value_state` is not empty, then we will use the value from the state.
                        label_up(&id);
                    }
                } else if !init_value.is_empty() {
                    // If `init_value` is not empty, then we will use the value from the `init_value`.
                    label_up(&id);
                }

                // Focus on the input if `init_focus` is true
                if init_focus {
                    focus_tag(&id);
                }

                || ()
            },
            (),
        );
    }

    {
        // If `value_state` is passed, then we will update the `value` state when the `value_state` is updated.
        if let Some(value_state) = value_state.clone() {
            let value = value.clone();
            use_effect_with_deps(
                move |value_state| {
                    // If the `value` is not equal to the `value_state`, then we will update the `value` state.
                    if value.to_string() != value_state.to_string() {
                        value.set(value_state.to_string());
                    }

                    || ()
                },
                value_state,
            );
        }
    }

    {
        // If `value_state` is passed, then we will update the `value_state` when the `value` is updated.
        let value = value.clone();
        use_effect_with_deps(
            move |value: &UseStateHandle<String>| {
                if let Some(value_state) = value_state {
                    // If the `value` is not equal to the `value_state`, then we will update the `value_state`.
                    if value_state.to_string() != value.to_string() {
                        value_state.set(value.to_string());
                    }
                }

                || ()
            },
            value,
        );
    }

    html! {
        <>
            {
                if paste {
                    html! {
                        <img
                            src="icons/paste.svg"
                            alt="Paste"
                            title="Paste"
                            onclick={
                                let value = value.clone();
                                let id = id.clone();
                                move |_| {
                                    let value = value.clone();
                                    let id = id.clone();
                                    spawn_local(async move {
                                        let js_text = get_from_clipboard().await.unwrap();
                                        let text = js_text.as_string().unwrap();

                                        value.set(text);
                                        label_up(&id);

                                    })
                                }
                            }
                        />
                    }
                } else {
                    html! {}
                }
            }
            <input
                {onkeyup}
                {id}
                type={format!("{}", input_type)}
                value={
                    if permission != InputPermission::Disabled {
                        (*value).clone()
                    } else {
                        String::new()
                    }
                }
                disabled={permission == InputPermission::Disabled}
                readonly={permission == InputPermission::ReadOnly}
                onblur={handle_blur_event}
                onfocus={handle_focus_event}
            />
        </>
    }
}
