use crate::*;

/// Props for the [`Label`] component.
#[derive(Properties, PartialEq)]
pub struct LabelProps {
    /// Label text.
    pub text: String,
    /// Children of the label (if any).
    ///
    /// You can use any html element as a child such as `<span>`, `<div>`, `<p>` etc.
    #[prop_or_default]
    pub children: Children,
}

/// Label component.
///
/// It is a child component of [`InputDiv`].
///
/// If you are using the [`Form`] component, then you should use this component instead of using the `<label>` tag.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`LabelProps`]
#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    // the class and id will come from the parent `InputDiv`
    let id = use_context::<super::InputId>().unwrap().0;

    html! {
        <>
            <label for={format!("input-{}", id)} class="label" id={format!("label-{}", id)}>
                {props.text.clone()}
                { for props.children.iter() }
            </label>
        </>
    }
}
