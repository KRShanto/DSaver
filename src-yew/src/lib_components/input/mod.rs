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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(self) struct InputId(String);

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct InputProps {
    #[prop_or_default]
    pub value_state: Option<UseStateHandle<String>>,
    #[prop_or_default]
    pub options: UseInputOptions,
    #[prop_or_default]
    pub init_value: String,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    // NOTE: If you give init_value and value_state, then value_state will be used to display the input

    let InputProps {
        value_state,
        options,
        init_value,
    } = (*props).clone();

    // the class and id will come from the parent `InputDiv`
    let id = use_context::<InputId>().unwrap().0;

    let UseInputOptions {
        input_type,
        permission,
    } = options.clone();

    let (value, onkeyup) = use_input(String::new(), &options);

    {
        let value_state = value_state.clone();
        let value = value.clone();
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(state) = value_state {
                    if !(*state).is_empty() {
                        label_up(&format!("input-{}", id));
                        value.set((*state).clone());
                    }
                } else if !init_value.is_empty() {
                    label_up(&format!("input-{}", id));
                    value.set(init_value.clone());
                }

                || ()
            },
            (),
        );
    }

    {
        let value_state = value_state.clone();
        let value = value.clone();
        use_effect_with_deps(
            move |value: &UseStateHandle<String>| {
                console_log!(format!("VALUE: {:?}", &value));

                if let Some(state) = value_state {
                    state.set((**value).clone());
                }

                || ()
            },
            value,
        );
    }

    html! {
        <>
            <input
                {onkeyup}
                id={format!("input-{}", id)}
                type={format!("{}", input_type)}
                value={
                    if permission != InputPermission::Disabled {
                        if let Some(state) = value_state {
                            (*state).clone()
                        } else {
                            (*value).clone()
                        }
                    } else {
                        String::new()
                    }
                }
                disabled={permission == InputPermission::Disabled}
                onblur={handle_blur_event}
                onfocus={handle_focus_event}

            />
        </>
    }
}
