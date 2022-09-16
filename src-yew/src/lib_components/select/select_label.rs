use crate::*;

#[derive(Properties, Eq, PartialEq)]
pub struct SelectLabelProps {
    pub text: String,
}

#[function_component(SelectLabel)]
pub fn select_label(props: &SelectLabelProps) -> Html {
    let text = props.text.clone();

    html! {
        <>
            <p class="label">{text}</p>
        </>
    }
}
