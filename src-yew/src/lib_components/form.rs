use crate::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    pub id: String,
    // pub title: String,
    pub button_text: String,
    pub onclick: Callback<MouseEvent>,
    pub children: ChildrenRenderer<FormPropsChildren>,
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum FormPropsChildren {
    InputWrapper(VChild<InputWrapper>),
    Select(VChild<Select>),
}

#[allow(clippy::from_over_into)]
impl Into<Html> for FormPropsChildren {
    fn into(self) -> Html {
        match self {
            Self::InputWrapper(child) => child.into(),
            Self::Select(child) => child.into(),
        }
    }
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let FormProps {
        // title,
        id,
        button_text,
        onclick,
        children,
    } = (*props).clone();

    html! {
        <>
            <div class={format!("form {}-form ", id)} id={format!("{}-form", id)}>
                // <h1 class="form-title">{title}</h1>

                { for children.iter() }

                <div class="option-buttons">
                    <button class="submit" {onclick}>{button_text}</button>
                </div>
            </div>
        </>
    }
}
