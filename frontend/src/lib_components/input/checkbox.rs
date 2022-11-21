use crate::*;

/// Props for the [`Checkbox`] component.
#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    /// Label/title of the checkbox.
    pub label_text: String,
    /// If the input's value is empty or not.
    ///
    /// It assumes that you are using this component for the input of a form.
    ///
    /// But if you are not using any input for this checkbox, you can set this to `true`.
    pub input_value_is_empty: bool,
    /// State for changing the checkbox's enabled/disabled state.
    pub disabled: UseStateHandle<bool>,
}
/// Checkbox component
///
/// It is a child component of [`InputWrapper`].
///
/// It makes a beautiful checkbox for you.
///
/// If you are using the [`Form`] component, then you should use this component instead of using the `<input type="checkbox">` tag.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`CheckboxProps`]
///
///
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        label_text,
        input_value_is_empty,
        disabled,
    } = (*props).clone();

    let onclick = {
        let disabled = disabled.clone();
        move |_| {
            // When this checkbox is clicked, the disabled state will be toggled
            // If the input has a value, then the checkbox won't be disabled
            if !*disabled {
                if input_value_is_empty {
                    // If the input has no value, then the checkbox will be disabled
                    disabled.set(true);
                }
            } else {
                // If the checkbox is disabled, then it will be enabled
                disabled.set(false);
            }
        }
    };

    html! {
        <>
            <div class="checkbox" {onclick}>
                <div class="checkmark">{
                    // If the checkbox is enabled, show a checkmark
                    if *disabled {
                        html! {
                            <img class="checked" src="icons/checked.svg" alt="Check mark" />
                        }
                    }
                    // If the checkbox is disabled, show a cross
                    else {
                        html! {
                            <img class="unchecked" src="icons/unchecked.svg" alt="Cross mark" />
                        }
                    }
                }</div>
                <p class="checkmark-title">{label_text}</p>
            </div>
        </>
    }
}
