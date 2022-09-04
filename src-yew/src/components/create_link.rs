use crate::*;
use itertools::Itertools;

#[function_component(CreateLink)]
pub fn new() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let create_link_state = use_context::<CreateLinkState>().unwrap().0;

    let display_error_state = use_context::<DisplayErrorState>().unwrap().0;
    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    let url_ref = NodeRef::default();
    let title_ref = NodeRef::default();
    let tags_ref = NodeRef::default();
    let priority_ref = NodeRef::default();
    let browser_ref = NodeRef::default();

    let title_disabled = use_state(|| true);

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

    html! {
        <div>
            <input type="text" ref={url_ref.clone()} placeholder="Url of the website" value="google.com"/>
            <br />
            <input type="text" ref={title_ref.clone()} placeholder="Title of the website" disabled={*title_disabled}/>
            <br />
            <div class="checkbox" onclick={
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
            <br />
            <input type="text" ref={tags_ref.clone()} placeholder="Tags" value="Google"/>
            <br />
            <input type="text" ref={priority_ref.clone()} placeholder="priority" value="A"/>
            <br />
            <input type="text" ref={browser_ref.clone()} placeholder="Browser" value="Firefox"/>
            <br />

            <button onclick={onclick}>{"Add"}</button>
        </div>


    }
}
