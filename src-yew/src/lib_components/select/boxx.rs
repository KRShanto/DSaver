use crate::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BoxProps {
    #[prop_or_default]
    pub init_value: Option<String>,
    pub list: Vec<String>,
    pub class: String,
    pub id: String,
    pub value_state: UseStateHandle<String>,
}

#[function_component(Box)]
pub fn boxx(props: &BoxProps) -> Html {
    let BoxProps {
        init_value,
        list,
        id,
        class,
        value_state,
    } = (*props).clone();

    let hide_or_show_class = use_state(|| "");
    let button_clicked_for_render = use_state(|| false);
    let render_div = use_state(|| false);
    let list_value = use_state(|| {
        if let Some(value) = init_value {
            value
        } else if !(*value_state).is_empty() {
            (*value_state).clone()
        } else {
            list[0].clone()
        }
    });

    {
        let button_clicked_for_render = button_clicked_for_render.clone();
        let hide_or_show_class = hide_or_show_class.clone();
        let render_div = render_div.clone();
        use_effect_with_deps(
            move |button_clicked_for_render| {
                // TODO: give docs
                if **button_clicked_for_render {
                    // the button has clicked to open the priority div
                    render_div.set(true);

                    set_timeout(
                        move || {
                            hide_or_show_class.set("show");
                        },
                        200,
                    )
                    .unwrap();
                } else {
                    // the button has clicked to close the priority div
                    hide_or_show_class.set("hide");

                    set_timeout(
                        move || {
                            render_div.set(false);
                        },
                        200,
                    )
                    .unwrap();
                }

                || ()
            },
            button_clicked_for_render,
        );
    }

    {
        let render_div = render_div.clone();
        let button_clicked_for_render = button_clicked_for_render.clone();
        let id = id.clone();
        use_effect_with_deps(
            move |render_div| {
                let remove_button_not_clicked = if **render_div {
                    let button_not_clicked = callback(move || {
                        button_clicked_for_render.set(false);
                    });

                    let remove_button_not_clicked =
                        if_not_clicked(&id, button_not_clicked.as_ref().unchecked_ref());

                    (Some(remove_button_not_clicked), Some(button_not_clicked))
                } else {
                    (None, None)
                };

                || {
                    // remove the event listener when the component is unmounted
                    if let Some(remove_button_not_clicked) = remove_button_not_clicked.0 {
                        remove_button_not_clicked
                            .call0(&JsValue::UNDEFINED)
                            .unwrap();
                    }

                    if let Some(button_not_clicked) = remove_button_not_clicked.1 {
                        button_not_clicked.forget();
                    }
                }
            },
            render_div,
        );
    }

    {
        let list_value = list_value.clone();
        use_effect_with_deps(
            move |list_value| {
                value_state.set((**list_value).clone());
                || ()
            },
            list_value,
        )
    }

    html! {
        <>
           <div class={classes!("select-div", class)}>
                <button class="select-button option" {id} onclick={
                    let button_clicked_for_render = button_clicked_for_render.clone();
                    let render_div = render_div.clone();
                    move |_| {
                        // if the `.option-div` aleady opened, then close it else open it
                        if *render_div {
                            button_clicked_for_render.set(false);
                        } else {
                            button_clicked_for_render.set(true);
                        }
                    }
                }>{&*list_value}</button>

                if *render_div {
                    <div class={format!("option-div {}", *hide_or_show_class)}>
                    {
                        list.into_iter().map(|p| {
                            html! {
                                <div class="option" onclick={
                                    let button_clicked_for_render = button_clicked_for_render.clone();
                                    let list_value = list_value.clone();
                                    let p = p.clone();
                                    move |_| {
                                        button_clicked_for_render.set(false);
                                        list_value.set(p.clone());
                                    }
                                }>{p}</div>
                            }
                        }).collect::<Html>()
                    }
                    </div>
                } else {
                    <></>
                }
            </div>
        </>
    }
}
