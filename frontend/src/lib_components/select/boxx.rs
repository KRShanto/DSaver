use crate::*;

/// Props for [`Box`] component.
#[derive(Properties, PartialEq, Clone)]
pub struct BoxProps {
    /// The list of options to choose from.
    pub list: Vec<String>,
    /// The class name of the box.
    pub class: String,
    /// The id of the box.
    pub id: String,
    /// The state of the value of the select box.
    ///
    /// This is a state that is used to store the value of the select box.
    ///
    /// If you need to update the input value by another way, then you can use this state.
    ///
    /// It is optional. If you don't need to update the input value from outside, then you don't need to use this.
    pub value_state: UseStateHandle<String>,
}

/// The select box component.
///
/// It is an alternative to the `<option>` tag.
///
/// It will render a button with an initial value, and when the user clicks on the button, it will show a list of options.
///
/// The user can choose an option from the list, and the button will show the chosen option.
///
/// It is a child of the [`Select`] component.
///
/// `props` - [`BoxProps`]
#[function_component(Box)]
pub fn boxx(props: &BoxProps) -> Html {
    let BoxProps {
        list,
        id,
        class,
        value_state,
    } = (*props).clone();

    // Whether the list is shown or not (with css)
    // Its a class, this state is only for the css
    let hide_or_show_class = use_state(|| "");
    // Is the button has clicked for the rendering the list or to close the list?
    let button_clicked_for_render = use_state(|| false);
    // Will the list be rendered?
    let render_div = use_state(|| false);
    // The value of the list
    // Initially, if `value_state` is given and it has a value, then use that value.
    // Otherwise, use the `init_value`.
    // Otherwise, use the first value of the `list`.
    let list_value = use_state(|| {
        if !(*value_state).is_empty() {
            (*value_state).clone()
        } else {
            list[0].clone()
        }
    });

    {
        let button_clicked_for_render = button_clicked_for_render.clone();
        let hide_or_show_class = hide_or_show_class.clone();
        let render_div = render_div.clone();
        // Show or hide the list
        use_effect_with_deps(
            move |button_clicked_for_render| {
                if **button_clicked_for_render {
                    // The button has clicked to open the priority div
                    // When the button is clicked for render, then we will show the list
                    render_div.set(true);

                    // The list has spawned, but it is hidden
                    // For making an animation, we will show the list (add the class `show`) after 200ms
                    // Else the list will be shown instantly (without animation)
                    set_timeout(
                        move || {
                            hide_or_show_class.set("show");
                        },
                        200,
                    )
                    .unwrap();
                } else {
                    // The button has clicked to close the priority div
                    // When the button is clicked for hide, then we will hide the list
                    hide_or_show_class.set("hide");

                    // For making an animation, First we will hide the list from css (replace the class `show` with `hide`)
                    // Then after 200ms, we will remove the list from the DOM
                    // Else the list will be hidden instantly (without animation)
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
        // Hide the list when the user clicks outside the list
        use_effect_with_deps(
            move |render_div| {
                // If the `render_div` is true, it means that the list is rendered
                // So when the `render_div` is true, we will add an event listener to the document to listen if the user clicks outside the Box.
                // If the user clicks outside the Box, then we will hide the list.
                // If the `render_div` is false, it means that the list is not rendered, so do nothing.

                let remove_button_not_clicked = if **render_div {
                    // A callback when the button is not clicked or user has clicked outside the Box
                    let button_not_clicked = callback(move || {
                        button_clicked_for_render.set(false);
                    });

                    // Pass the callback to the javascript function `if_not_clicked`.
                    // That function will add an event listener to the document to listen if the user clicks outside the Box.
                    // If the user clicks outside the Box, then it will call the callback.
                    // It will return a function to remove the event listener.
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

                    // remove the callback when the component is unmounted
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
        // When the `list_value` is changed, then update the `value_state`
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
