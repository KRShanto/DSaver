use crate::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub text: String,
    #[prop_or_default]
    pub children: Children,
}

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
