use crate::*;

#[derive(Properties, PartialEq)]
pub struct InputDivProps {
    pub children: ChildrenRenderer<InputDivPropsChildren>,
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum InputDivPropsChildren {
    Label(VChild<Label>),
    Input(VChild<Input>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for InputDivPropsChildren {
    fn into(self) -> Html {
        match self {
            Self::Label(child) => child.into(),
            Self::Input(child) => child.into(),
        }
    }
}

#[function_component(InputDiv)]
pub fn input_div(props: &InputDivProps) -> Html {
    html! {
        <>
            <div class="label-input">
                { for props.children.iter() }
            </div>
        </>
    }
}
