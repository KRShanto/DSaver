use crate::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    pub class: String,
    pub label_text: String,
    pub input_value_is_empty: bool,
    pub disabled: UseStateHandle<bool>,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        class,
        label_text,
        input_value_is_empty,
        disabled,
    } = (*props).clone();

    html! {
        <>
            <div
                class="checkbox"
                onclick={
                    let disabled = disabled.clone();
                    move |_| {
                        if !*disabled {
                            if input_value_is_empty {
                                // if the input's value is empty then disable the input
                                disabled.set(true);
                            }
                        } else {
                            disabled.set(false);
                        }
                    }
                }>
                <div class="checkmark">{
                    if *disabled {
                        html! {
                            <img class="checked" src="icons/checked.svg" alt="Check mark" />
                        }
                    } else {
                        html! {
                            <img class="unchecked" src="icons/unchecked.svg" alt="Cross mark" />
                        }
                    }
                }</div>
                <p class={format!("checkmark-{}", class)}>{label_text}</p>
            </div>
        </>
    }
}
