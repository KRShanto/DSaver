use crate::*;

mod boxx;
mod select_label;

pub use boxx::*;
pub use select_label::*;

/// Props of the [`Select`] component.
#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub children: ChildrenRenderer<SelectPropsChildren>,
}

/// Children of the [`Select`] component.
#[derive(Clone, derive_more::From, PartialEq)]
pub enum SelectPropsChildren {
    Label(VChild<SelectLabel>),
    Box(VChild<Box>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for SelectPropsChildren {
    fn into(self) -> Html {
        match self {
            Self::Label(child) => child.into(),
            Self::Box(child) => child.into(),
        }
    }
}

/// The select component.
///
/// It is an alternative to the `<select>` tag.
///
/// It renders a label and a select box.
///
/// When the user clicks on the select box, it will show a list of options.
///
/// The user can choose an option from the list, and the select box will show the chosen option.
///
/// If you are using the [`Form`] component, then you should this component instead of the `<select>` tag.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`SelectProps`]
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    html! {
        <>
        <div class="form-wrapper select-form" >
            { for props.children.iter() }
        </div>
        </>
    }
}
