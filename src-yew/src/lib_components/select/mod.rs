use crate::*;

mod boxx;
mod select_label;

pub use boxx::*;
pub use select_label::*;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub children: ChildrenRenderer<SelectPropsChildren>,
}

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
