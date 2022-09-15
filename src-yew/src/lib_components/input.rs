use crate::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct InputProps {
    pub value_state: UseStateHandle<String>,
    #[prop_or_default]
    pub options: UseInputOptions,
    #[prop_or_default]
    pub value: String,
    pub class: String,
    pub id: String,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let InputProps {
        value_state,
        options,
        value,
        class,
        id,
    } = (*props).clone();

    let UseInputOptions {
        input_type,
        disabled,
    } = options.clone();

    let (value, onkeyup) = use_input(&value, &options);

    {
        let value = value.clone();
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
                {id}
                {class}
                {disabled}
                type={format!("{}", input_type)}
                value={(*value).clone()}
                onblur={handle_blur_event}
                onfocus={handle_focus_event}

            />
        </>
    }
}
