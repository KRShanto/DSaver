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
    pub value_state: UseStateHandle<String>,
    #[prop_or_default]
    pub options: UseInputOptions,
    #[prop_or_default]
    pub init_value: String,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        value_state,
        options,
        init_value,
    } = (*props).clone();

    // the class and id will come from the parent `InputDiv`
    let id = use_context::<InputId>().unwrap().0;

    let UseInputOptions {
        input_type,
        disabled,
    } = options.clone();

    let (value, onkeyup) = use_input(&init_value, &options);

    {
        let value_state = value_state.clone();
        use_effect_with_deps(
            move |value| {
                value_state.set((**value).clone());
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
                {disabled}
                type={format!("{}", input_type)}
                value={(*value_state).clone()}
                onblur={handle_blur_event}
                onfocus={handle_focus_event}

            />

        </>
    }
}
