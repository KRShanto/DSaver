use crate::*;
use itertools::Itertools;
use wasm_bindgen::JsCast;

#[function_component(CreateLink)]
pub fn new() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let create_link_state = use_context::<CreateLinkState>().unwrap().0;

    let display_error_state = use_context::<DisplayErrorState>().unwrap().0;
    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    let url_ref = NodeRef::default();
    let title_ref = NodeRef::default();
    let tags_ref = NodeRef::default();
    let priority_ref = NodeRef::default();
    let browser_ref = NodeRef::default();

    let title_disabled = use_state(|| true);

    // states for tags
    let tags_value = use_state(String::new);
    let previously_matched_tags = use_state(Vec::new);

    let onclick = {
        let url_ref = url_ref.clone();
        let title_ref = title_ref.clone();
        let tags_ref = tags_ref.clone();
        let priority_ref = priority_ref.clone();
        let browser_ref = browser_ref.clone();

        move |_| {
            let url = url_ref.cast::<HtmlInputElement>().unwrap().value();
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            let tags = tags_ref.cast::<HtmlInputElement>().unwrap().value();
            let priority = priority_ref.cast::<HtmlInputElement>().unwrap().value();
            let browser = browser_ref.cast::<HtmlInputElement>().unwrap().value();
            // TODO: trim() these
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
                .priority(priority.chars().next().unwrap())
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

    let tags_onchange = {
        let tags_ref = tags_ref.clone();
        let tags_value = tags_value.clone();
        let previously_matched_tags = previously_matched_tags.clone();
        move |_| {
            let tags = tags_ref.cast::<HtmlInputElement>().unwrap().value();

            match tags.chars().last() {
                // if the last character is blank, then do not show any tags suggestion
                Some(tag) => {
                    if tag.to_string() == " " {
                        previously_matched_tags.set(Vec::new());
                    } else {
                        // get the current word / last word and find which tags are matched.
                        // NOTE: the matched tags are only for current word.
                        let current_word = tags.split_whitespace().last().unwrap_or("");

                        let mut tags_vec = Vec::new();

                        // loop tags
                        for tag in &*displayed_tags {
                            if tag.starts_with(current_word) {
                                tags_vec.push(tag.to_string());
                            }
                        }

                        previously_matched_tags.set(tags_vec);
                    }
                }
                None => previously_matched_tags.set(Vec::new()),
            }

            tags_value.set(tags);
        }
    };

    html! {
        <div class="create-link-form">
            <h1 class="form-title">{"Create a new link"}</h1>

            <div class="form-wrapper" id="create-url-wrapper">
                <label for="create-url">{"Url of the webpage"}</label>
                <br />
                <input
                    class="create-url"
                    id="create-url"
                    type="text"
                    ref={url_ref.clone()}
                />
            </div>

            <div class="form-wrapper" id="create-title-wrapper">
                <label for="create-title">{"Title of the webpage"}</label>
                <br />
                <input
                    class="create-title"
                    id="create-title"
                    type="text"
                    ref={title_ref.clone()}
                    disabled={*title_disabled}
                />
                <div
                    class="checkbox"
                    onclick={
                        let title_disabled = title_disabled.clone();
                        move |_| {
                            title_disabled.set(!*title_disabled);
                        }
                    }>
                    <p>{"Get the title from the webpage"}</p>
                    <div class="checkmark">{
                        if *title_disabled {
                            "✓"
                        } else {
                            "X"
                        }
                    }</div>
                </div>
            </div>

            <div class="form-wrapper" id="create-tags-wrapper">
                <label for="create-tags">{"Tags (separate with spaces)"}</label>
                if tags_value.is_empty() {
                    <></>
                } else {
                    <div class="tags">
                        {
                            tags_value.split_whitespace().map(|tag| {
                                html! {
                                    <span>{tag}</span>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                }
                <br />
                <input
                    class="create-tags"
                    id="create-tags"
                    type="text"
                    ref={tags_ref.clone()}
                    value={(*tags_value).clone()}
                    onkeyup={tags_onchange}
                />

                <div class="previous-tags">
                    <span class="title">{"Previous tags"}</span>
                    {
                        previously_matched_tags.iter().map(|tag| {
                            html! {
                                <button>{tag}</button>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>

            <div class="form-wrapper" id="create-priority-wrapper">
                <label for="create-priority">{"Priority of the link"}</label>
                <br />
                <input
                    class="create-priority"
                    id="create-priority"
                    type="text"
                    ref={priority_ref.clone()}
                    value="A"
            />
            </div>

            <div class="form-wrapper" id="create-browser-wrapper">
                <label for="create-browser">{"From which browser you want to open this link"}</label>
                <br />
                <input
                    class="create-browser"
                    id="create-browser"
                    type="text"
                    ref={browser_ref.clone()}
                    value="Firefox"
                />
            </div>

            <button
                class="submit"
                onclick={onclick}
            >{"Add"}</button>
        </div>


    }
}
