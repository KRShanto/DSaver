use crate::*;
use itertools::Itertools;
use webru::{callback, set_timeout};

#[function_component(CreateLink)]
pub fn new() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let create_link_state = use_context::<CreateLinkState>().unwrap().0;

    let display_error_state = use_context::<DisplayErrorState>().unwrap().0;
    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    let (url_value, url_onkeyup) = use_input("");
    let (title_value, title_onkeyup) = use_input("");
    let (tags_value, tags_onkeyup) = use_input("");
    let priority_value = use_state(|| 'A');
    let browser_value = use_state(|| String::from("Default"));

    let title_disabled = use_state(|| true);

    // previously created tags || tags that matches tags from `displayed_tags`
    let previously_matched_tags = use_state(Vec::new);

    let priority_div_class = use_state(|| "");
    let priority_button_clicked_for_render = use_state(|| false);
    let render_priority_div = use_state(|| false);

    let browser_div_class = use_state(|| "");
    let browser_button_clicked_for_render = use_state(|| false);
    let render_browser_div = use_state(|| false);

    let priority_list = (b'A'..=b'Z').map(char::from);

    let form_hide = use_state(|| false);

    let onclick = {
        let url_value = url_value.clone();
        let title_value = title_value.clone();
        let tags_value = tags_value.clone();
        let priority_value = priority_value.clone();
        let browser_value = browser_value.clone();
        let create_link_state = create_link_state.clone();

        move |_| {
            let url = (*url_value).clone().trim().to_string();
            let title = (*title_value).clone().trim().to_string();
            let tags = (*tags_value).clone().trim().to_string();
            let priority = *priority_value;
            let browser = browser_value.clone().trim().to_string();
            let display_error_state = display_error_state.clone();
            let display_error_data = display_error_data.clone();

            let link = Link::new_with_date(url)
                .title(title.is_empty().then(|| None).unwrap_or(Some(title)))
                .tags(
                    tags.split_whitespace()
                        .map(|s| s.to_string())
                        .unique()
                        .collect(),
                )
                .priority(priority)
                .browser(Browser::from(browser));

            let links = links.clone();
            let create_link_state = create_link_state.clone();
            spawn_local(async move {
                // hide the component
                create_link_state.set(false);

                let new_link = add_data(
                    struct_to_string(&*links).unwrap(),
                    struct_to_string(&link).unwrap(),
                )
                .await
                .unwrap()
                .as_string()
                .unwrap();

                if let Ok(new_link) = string_to_struct::<Link>(&new_link) {
                    console_log!(format!("We found a new link: {:?}", new_link));
                    let mut old_links = (*links).clone();
                    old_links.push(new_link);

                    links.set(old_links);
                } else if let Ok(error) = string_to_struct::<ErrorReporter>(&new_link) {
                    // fill data for `DisplayError` component
                    display_error_data.set(Some(DisplayErrorInnerData {
                        error_reporter: error,
                        options_message: Some(
                            "You can still add the link to the collections. Do you want to add it?"
                                .to_string(),
                        ),
                        options_buttons: Some(vec![
                            (
                                String::from("Add"),
                                Callback::from({
                                    let display_error_state = display_error_state.clone();
                                    let display_error_data = display_error_data.clone();
                                    move |_| {
                                        // push the old (created by user) link to the cold collections
                                        let mut old_links = (*links).clone();
                                        old_links.push(link.clone());

                                        {
                                            let old_links = old_links.clone();
                                            spawn_local(async move {
                                                store_data(struct_to_string(&old_links).unwrap())
                                                    .await
                                                    .unwrap();
                                            });
                                        }

                                        links.set(old_links);

                                        // hide the component
                                        display_error_state.set(false);
                                        display_error_data.set(None);
                                    }
                                }),
                            ),
                            (
                                String::from("Cancel"),
                                Callback::from({
                                    let display_error_data = display_error_data.clone();
                                    let display_error_state = display_error_state.clone();
                                    move |_| {
                                        // hide the component
                                        display_error_state.set(false);
                                        display_error_data.set(None);
                                    }
                                }),
                            ),
                        ]),
                    }));

                    // display the component `DisplayError`
                    display_error_state.set(true);
                } else {
                    console_error!("Neither `Link` nor `ErrorReporter` was found")
                }
            });
        }
    };

    {
        let previously_matched_tags = previously_matched_tags.clone();
        use_effect_with_deps(
            move |tags| {
                let tags = (**tags).to_lowercase();

                match tags.chars().last() {
                    // if the last character is blank, then do not show any tags suggestion
                    Some(tag) => {
                        if tag.to_string() == " " {
                            previously_matched_tags.set(Vec::new());
                        } else {
                            // get the current word / last word and find which tags are matched.
                            // NOTE: the matched tags are only for current word.
                            let current_word = tags.split_whitespace().last().unwrap_or("");

                            let mut prev_tags_vec = Vec::new();

                            // loop tags
                            for dis_tag in &*displayed_tags {
                                if dis_tag.to_lowercase().contains(current_word) {
                                    prev_tags_vec.push(dis_tag.to_string());
                                }
                            }

                            previously_matched_tags.set(prev_tags_vec);
                        }
                    }
                    None => previously_matched_tags.set(Vec::new()),
                }

                || ()
            },
            tags_value.clone(),
        );
    }

    {
        let priority_button_clicked_for_render = priority_button_clicked_for_render.clone();
        let priority_div_class = priority_div_class.clone();
        let render_priority_div = render_priority_div.clone();
        use_effect_with_deps(
            move |button_clicked_for_render| {
                // TODO: give docs
                if **button_clicked_for_render {
                    // the button has clicked to open the priority div
                    render_priority_div.set(true);

                    set_timeout(
                        move || {
                            priority_div_class.set("show");
                        },
                        200,
                    )
                    .unwrap();
                } else {
                    // the button has clicked to close the priority div
                    priority_div_class.set("hide");

                    set_timeout(
                        move || {
                            render_priority_div.set(false);
                        },
                        200,
                    )
                    .unwrap();
                }

                || ()
            },
            priority_button_clicked_for_render,
        );
    }

    {
        let browser_button_clicked_for_render = browser_button_clicked_for_render.clone();
        let browser_div_class = browser_div_class.clone();
        let render_browser_div = render_browser_div.clone();
        use_effect_with_deps(
            move |button_clicked_for_render| {
                if **button_clicked_for_render {
                    // the button has clicked to open the browser div
                    render_browser_div.set(true);

                    set_timeout(
                        move || {
                            browser_div_class.set("show");
                        },
                        200,
                    )
                    .unwrap();
                } else {
                    // the button has clicked to close the browser div
                    browser_div_class.set("hide");

                    set_timeout(
                        move || {
                            render_browser_div.set(false);
                        },
                        200,
                    )
                    .unwrap();
                }

                || ()
            },
            browser_button_clicked_for_render,
        )
    }

    {
        let render_priority_div = render_priority_div.clone();
        let priority_button_clicked_for_render = priority_button_clicked_for_render.clone();
        use_effect_with_deps(
            |render_priority_div| {
                if **render_priority_div {
                    let priority_not_clicked = callback(move || {
                        priority_button_clicked_for_render.set(false);
                    });

                    if_not_clicked(
                        "priority-select-button",
                        priority_not_clicked.as_ref().unchecked_ref(),
                    );

                    priority_not_clicked.forget();
                }

                || {}
            },
            render_priority_div,
        );
    }

    {
        let render_browser_div = render_browser_div.clone();
        let browser_button_clicked_for_render = browser_button_clicked_for_render.clone();
        use_effect_with_deps(
            |render_browser_div| {
                if **render_browser_div {
                    let browser_not_clicked = callback(move || {
                        browser_button_clicked_for_render.set(false);
                    });

                    if_not_clicked(
                        "browser-select-button",
                        browser_not_clicked.as_ref().unchecked_ref(),
                    );

                    browser_not_clicked.forget();
                }

                || ()
            },
            render_browser_div,
        )
    }

    html! {
        <div class={format!("create-link-form {}", if *form_hide { "hide"} else {""})}>
            <h1 class="form-title">{"Create a new link"}</h1>

            <div class="form-wrapper" id="create-url-wrapper">
                <div class="label-input">
                    <label class="label" for="create-url" id="label-create-url">{"Url of the webpage"}</label>
                    <input
                        class="create-url"
                        id="create-url"
                        type="text"
                        value={(*url_value).clone()}
                        onkeyup={url_onkeyup}
                        onblur={handle_blur_event}
                        onfocus={handle_focus_event}
                    />
                </div>
            </div>

            <div class="form-wrapper" id="create-title-wrapper">
                <div class="label-input">
                    <label class="label" for="create-title" id="label-create-title">{"Title of the webpage"}</label>
                    <input
                        class="create-title"
                        id="create-title"
                        type="text"
                        value={(*title_value).clone()}
                        onkeyup={title_onkeyup}
                        onblur={handle_blur_event}
                        onfocus={handle_focus_event}
                        disabled={*title_disabled}
                    />
                </div>
                <div
                    class="checkbox"
                    onclick={
                        let title_disabled = title_disabled.clone();
                        move |_| {
                            if !*title_disabled {
                                if (*title_value).is_empty() {
                                    // if the title's value is empty then disable the input
                                    title_disabled.set(true);
                                }
                            } else {
                                title_disabled.set(false);
                            }
                        }
                    }>
                    <div class="checkmark">{
                        if *title_disabled {
                            html! {
                                <img class="checked" src="icons/checked.svg" alt="Check mark" />
                            }
                        } else {
                            html! {
                                <img class="unchecked" src="icons/unchecked.svg" alt="Cross mark" />
                            }
                        }
                    }</div>
                    <p class="checkmark-title">{"Get the title from the webpage"}</p>
                </div>
            </div>

            <div id="create-tags-wrapper" class="form-wrapper">
                <div class="label-input">
                    <label for="create-tags" class="label" id="label-create-tags">
                        {"Tags "}
                        <span>{"(separate with spaces)"}</span>
                    </label>
                    <input
                        class="create-tags"
                        id="create-tags"
                        type="text"
                        value={(*tags_value).clone()}
                        onkeyup={tags_onkeyup}
                        onblur={handle_blur_event}
                        onfocus={handle_focus_event}
                    />
                </div>
                if tags_value.is_empty() {
                    <></>
                } else {
                    <div class="current-tags">
                        <p class="title">{"Current Tags"}</p>
                        {
                            tags_value.split_whitespace().map(|tag| {
                                html! {
                                    <span class="tag">{tag}</span>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
                if previously_matched_tags.is_empty() {
                    <></>
                } else {
                    <div class="previous-tags">
                        <p class="title">{"Previous tags"}</p>
                        {
                            (*previously_matched_tags).iter().map(|tag| {
                                let tags_value = tags_value.clone();
                                let tag = tag.clone();
                                html! {
                                    <button class="tag" onclick={
                                        let tag = tag.clone();
                                        move |_| {
                                            // tags's value
                                            let previous_tags_value = (*tags_value).clone();

                                            // split the tags by whitespace
                                            let mut previous_tags_value_splitted: Vec<&str> =
                                                previous_tags_value.split_whitespace().collect();

                                            // replace the `tag` with the last element in the previous_tags_value_splitted
                                            previous_tags_value_splitted.pop();
                                            previous_tags_value_splitted.push(&tag);

                                            // set the tags_value to update the tag's value
                                            tags_value.set(previous_tags_value_splitted.join(" "));

                                            // focus on the input
                                            focus_tag();
                                        }
                                    }>{tag}</button>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
            </div>

            <div class="form-wrapper select-form" id="create-priority-wrapper">
                // TODO: use arrow icons
                <p class="label" id="label-create-priority">{"Priority of the link"}</p>

                <div class="priority-div select-div">
                    <button id="priority-select-button" class="priority select-button option" onclick={
                            let priority_button_clicked_for_render = priority_button_clicked_for_render.clone();
                            let render_priority_div = render_priority_div.clone();
                            move |_| {
                                // if the `.option-div` aleady opened, then close it else open it
                                if *render_priority_div {
                                    priority_button_clicked_for_render.set(false);
                                } else {
                                    priority_button_clicked_for_render.set(true);
                                }
                            }
                        }
                    >{&*priority_value}</button>

                    if *render_priority_div {
                        <div class={format!("option-div {}", *priority_div_class)}>
                        {
                            priority_list.into_iter().map(|p| {
                                html! {
                                    <div class="option" onclick={
                                        let priority_button_clicked_for_render = priority_button_clicked_for_render.clone();
                                        let priority_value = priority_value.clone();
                                        move |_| {
                                            priority_button_clicked_for_render.set(false);
                                            priority_value.set(p);
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

            </div>

            <div class="form-wrapper select-form" id="create-browser-wrapper">
                <p class="label" id="label-create-browser">{"From which browser you want to open this link"}</p>

                <div class="browser-div select-div">
                    <button id="browser-select-button" class="browser select-button option" onclick={
                            let browser_button_clicked_for_render = browser_button_clicked_for_render.clone();
                            let render_browser_div = render_browser_div.clone();
                            move |_| {
                                // if the `.option-div` aleady opened, then close it else open it
                                if *render_browser_div {
                                    browser_button_clicked_for_render.set(false);
                                } else {
                                    browser_button_clicked_for_render.set(true);
                                }
                            }
                        }
                    >{&*browser_value}</button>

                    if *render_browser_div {
                        <div class={format!("option-div {}", *browser_div_class)}>
                        {
                            Browser::get_vec().into_iter().map(|browser| {
                                html! {
                                    <div class="option" onclick={
                                        let browser_button_clicked_for_render = browser_button_clicked_for_render.clone();
                                        let browser_value = browser_value.clone();
                                        let browser = browser.clone();
                                        move |_| {
                                            browser_button_clicked_for_render.set(false);
                                            browser_value.set(browser.clone());
                                        }
                                    }>{&browser}</div>
                                }
                            }).collect::<Html>()
                        }
                        </div>
                    } else {
                        <></>
                    }
                </div>
            </div>

            <div class="option-buttons">
                <button class="submit" onclick={onclick}>{"Add"}</button>
                <button class="cancel" onclick={
                    let form_hide = form_hide.clone();
                    move |_| {
                        let create_link_state = create_link_state.clone();
                        set_timeout(
                            move || {
                                create_link_state.set(false);
                            },
                            1000, // removing the form after 1sec because some animations might be are happening
                        )
                        .unwrap();

                        form_hide.set(true);
                    }
                }>{"Cancel"}</button>
            </div>
        </div>


    }
}
