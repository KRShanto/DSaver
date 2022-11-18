use super::InputId;
use crate::*;

/// Prop of the [`InputWrapper`] component.
#[derive(Properties, PartialEq)]
pub struct InputWrapperProps {
    /// Id of the input and labels.
    pub id: String,
    /// Children
    pub children: ChildrenRenderer<InputWrapperPropsChildren>,
}

/// Children of the [`InputWrapper`] component.
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

/// Wrapper component for the [`InputDiv`], [`InputTags`] and [`Checkbox`] components.
///
/// It is a child component of [`Form`]. And it is a parent of [`InputDiv`], [`InputTags`] and [`Checkbox`] components.
///
/// It takes an `id` prop and pass that to the children components.
///
/// If you are using the [`Form`] component, then you have to use this component to use the [`InputDiv`], [`InputTags`] and [`Checkbox`] components.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`InputWrapperProps`]
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
