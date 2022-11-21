use crate::*;

/// Props for the [`SelectLabel`] component.
#[derive(Properties, Eq, PartialEq)]
pub struct SelectLabelProps {
    pub text: String,
}

/// Label for the [`Select`] component.
///
/// It just displays a text.
///
/// It is a child of the [`Select`] component.
///
/// `props` - [`SelectLabelProps`]
#[function_component(SelectLabel)]
pub fn select_label(props: &SelectLabelProps) -> Html {
    let text = props.text.clone();

    html! {
        <>
            <p class="label">{text}</p>
        </>
    }
}
