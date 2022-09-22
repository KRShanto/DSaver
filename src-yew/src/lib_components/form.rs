use crate::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FormProps {
    pub id: String,
    pub title: String,
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
        title,
        id,
        button_text,
        onclick,
        children,
    } = (*props).clone();

    let form_hide = use_state(|| false);
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;

    {
        let form_hide = form_hide.clone();
        use_effect_with_deps(
            move |form_hide| {
                if **form_hide {
                    set_timeout(
                        move || {
                            // form_render_state.set(false);
                            popup_box_state.set(PopupBox::None);
                        },
                        1000, // removing the form after 1sec because some animations might be are happening
                    )
                    .unwrap();
                }

                || ()
            },
            form_hide,
        );
    }

    html! {
        <>
            <div class={format!("form {}-form {}", id, if *form_hide { "hide"} else {""} )} id={format!("{}-form", id)}>
                <h1 class="form-title">{title}</h1>

                { for children.iter() }

                <div class="option-buttons">
                    <button class="submit" {onclick}>{button_text}</button>
                    <button class="cancel" onclick={
                        let form_hide = form_hide.clone();
                        move |_| {
                            form_hide.set(true);
                        }
                    }>{"Cancel"}</button>
                </div>
            </div>
        </>
    }
}
