use crate::*;

/// Props for the [`InputDiv`] component.
#[derive(Properties, PartialEq)]
pub struct InputDivProps {
    pub children: ChildrenRenderer<InputDivPropsChildren>,
}

/// InputDiv's children
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

/// Wrapper component for the `<input>` and `<label>` tags.
///
/// It is a child component of [`InputWrapper`]. And it is a parent of [`Label`] and [`Input`] components.
///
/// If you are using the [`Form`] component, then you should use this component instead of using the `<input>` tag.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`InputDivProps`]
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
