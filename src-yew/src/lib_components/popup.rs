use crate::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    #[prop_or_default]
    pub classes: Vec<String>,
    pub id: String,
    pub title: String,
    pub children: Children,
}

/// If you want to change the styles of the popup for a specific area/child, then you can target that popup using the `popup-{id}` id.
#[function_component(Popup)]
pub fn p(props: &PopupProps) -> Html {
    let popup_hide_state = use_context::<PopupBoxHideState>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;
    let popup_box_ready_state = use_context::<PopupBoxReadyState>().unwrap().0;

    {
        let popup_hide_state = popup_hide_state.clone();
        use_effect_with_deps(
            move |popup_hide_state| {
                if **popup_hide_state {
                    let popup_hide_state = popup_hide_state.clone();
                    set_timeout(
                        move || {
                            popup_box_state.set(PopupBox::None);
                            popup_hide_state.set(false);
                        },
                        300, // removing the form after 1sec because some animations might be are happening
                    )
                    .unwrap();
                }

                || ()
            },
            popup_hide_state,
        );
    }
    {
        let popup_box_ready_state = popup_box_ready_state.clone();
        use_effect_with_deps(
            move |_| {
                let timeout_id = {
                    let popup_box_ready_state = popup_box_ready_state.clone();
                    set_timeout(
                        move || {
                            popup_box_ready_state.set(true);
                        },
                        100, // useing a timeout. bcz if we change this immediately, it won't be beneficial.
                    )
                    .unwrap()
                };

                move || {
                    popup_box_ready_state.set(false);
                    clear_timeout(timeout_id);
                }
            },
            (),
        );
    }
    html! {
        if *popup_box_ready_state {
            <div
                class={classes!(
                    "popup",
                    props.classes.clone(),
                    if *popup_hide_state { "hide" } else {""}
                )}
                id={format!("popup-{}", &props.id)}
            >
                <img
                    class="cancel"
                    src="icons/cross.svg"
                    onclick={
                        let popup_hide_state = popup_hide_state.clone();
                        move |_| {
                            popup_hide_state.set(true);
                        }
                    }
                />

                <h1 class="popup-title">{&props.title}</h1>

                {props.children.clone()}
            </div>
        }
    }
}
