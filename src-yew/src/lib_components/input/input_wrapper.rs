use super::InputId;
use crate::*;

#[derive(Properties, PartialEq)]
pub struct InputWrapperProps {
    pub id: String,
    pub children: ChildrenRenderer<InputWrapperPropsChildren>,
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum InputWrapperPropsChildren {
    InputDiv(VChild<InputDiv>),
    InputTags(VChild<InputTags>),
    Checkbox(VChild<Checkbox>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for InputWrapperPropsChildren {
    fn into(self) -> Html {
        match self {
            Self::Checkbox(child) => child.into(),
            Self::InputDiv(child) => child.into(),
            Self::InputTags(child) => child.into(),
        }
    }
}

#[function_component(InputWrapper)]
pub fn input_wrapper(props: &InputWrapperProps) -> Html {
    let id = props.id.clone();

    html! {
        <>
            <ContextProvider<InputId> context={InputId(id.clone())}>
                <div class="form-wrapper" id={format!("{}-wrapper", id)}>
                    { for props.children.iter() }
                </div>
            </ContextProvider<InputId>>
        </>
    }
}
