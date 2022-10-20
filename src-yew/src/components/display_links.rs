use crate::*;
use itertools::Itertools;

#[function_component(DisplayLinks)]
pub fn show_links() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let editing_link_id = use_context::<EditingLinkIdState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;
    let popup_box_ready_state = use_context::<PopupBoxReadyState>().unwrap().0;
    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    // links to display
    let mut displayed_links_for_tags = Vec::new();

    // looping `links`'s tags with `displayed_tags` and if any `links.tags` match with `displayed_tags`,
    // then the `link` will be pushed into `displayed_links_for_tags`.
    for display_tag in (*displayed_tags).clone() {
        (*links).iter().for_each(|link| {
            for tag in &link.tags {
                if &display_tag == tag {
                    displayed_links_for_tags.push(link.clone());
                    break;
                }
            }
        });
    }

    let mut displayed_links_for_browsers = Vec::new();

    // now remove those links whose browsers != displayed_browsers
    for browser in (*displayed_browsers).clone() {
        displayed_links_for_tags.iter().for_each(|link| {
            if link.browser == browser {
                displayed_links_for_browsers.push(link.clone());
            }
        })
    }

    let displayed_links = displayed_links_for_browsers.into_iter().unique();

    let mut prirorities = Vec::new();

    for link in &*links {
        prirorities.push(link.priority);
    }

    prirorities.sort();
    prirorities.dedup();

    let mut i = 0;

    let mut display_link_index = 0;

    let display_link_body: UseStateHandle<Option<i32>> = use_state(|| None);

    html! {
        <>
        <div class="display-links" id="display-links" onclick={
            let popup_box_state = popup_box_state.clone();
            move |_| {
                if *popup_box_ready_state && (*popup_box_state).clone() != PopupBox::None {
                    popup_box_state.set(PopupBox::None);
                }
            }
        }>
            {
            prirorities.iter().map(|priority| {
                i+=1;

                // get the links according to the priority
                let links_to_show = displayed_links.clone().into_iter().filter(|link| &link.priority == priority);

                html! {
                    <>
                    if links_to_show.clone().next().is_some() {
                        <div class={classes!(
                            "display-links-container",
                            if i % 2 == 0 {
                                "red"
                            } else {
                                "orange"
                            }
                        )}>
                            <div class="right-side" />
                            <div class="left-side">
                                <div class="priority">
                                    <h2 class="text">{priority}</h2>
                                </div>
                                {
                                    links_to_show.clone().into_iter().map(|link| {
                                        display_link_index += 1;

                                        html! {
                                            <div class="link">
                                                <div class="link-head">
                                                    <div class="title-area" ondblclick={
                                                        let browser = link.browser.clone();
                                                        let url = link.url.clone();
                                                        let display_link_body = display_link_body.clone();
                                                        let display_error_data = display_error_data.clone();
                                                        let popup_box_state = popup_box_state.clone();

                                                        move |_| {
                                                            // if current link is not opened, then open the browser on dblclick
                                                            if let Some(display) = *display_link_body {
                                                                if display != (display_link_index - 1) {
                                                                    open_user_browser(url.clone(), browser.clone(), display_error_data.clone(), popup_box_state.clone());
                                                                }
                                                            } else {
                                                                open_user_browser(url.clone(), browser.clone(),display_error_data.clone(), popup_box_state.clone());
                                                            }
                                                        }
                                                }   >
                                                        <h3 class="title">{link.title.clone().unwrap()}</h3>
                                                    </div>
                                                    <div class="icon" onclick={
                                                        let display_link_body = display_link_body.clone();
                                                        move |_| {
                                                            if let Some(display) = *display_link_body {
                                                                if display == (display_link_index - 1) {
                                                                    display_link_body.set(None);
                                                                } else {
                                                                    display_link_body.set(Some(display_link_index - 1));
                                                                }
                                                            } else {
                                                                display_link_body.set(Some(display_link_index - 1));
                                                            }

                                                        }
                                                    }>
                                                        if let Some(display) = *display_link_body {
                                                            if display == (display_link_index - 1) {
                                                                <img class="up" src="icons/up-arrow.svg" alt="Up arrow" />
                                                            } else {
                                                                <img class="down" src="icons/down-arrow.svg" alt="Down arrow" />
                                                            }
                                                        } else {
                                                            <img class="down" src="icons/down-arrow.svg" alt="Down arrow" />
                                                        }
                                                    </div>
                                                </div>
                                                <div class={classes!(
                                                    "link-body",
                                                    if let Some(display) = *display_link_body {
                                                        if display == (display_link_index - 1) {
                                                            "display"
                                                        } else {
                                                            ""
                                                        }
                                                    } else {
                                                        ""
                                                    }
                                                )}>
                                                    <div class="info">
                                                        <p class="url">{&link.url}</p>
                                                        // TODO: copy button
                                                        <p class="description">{link.description.clone().unwrap()}</p>
                                                        <ul class="tags">
                                                            {
                                                            link.tags.iter().map(|tag| {
                                                                html! {
                                                                        <li class="tag">{tag}</li>
                                                                    }
                                                                }).collect::<Html>()
                                                            }
                                                        </ul>
                                                        <p class="date">{&link.date}</p>
                                                    </div>
                                                    <div class="options">
                                                        <button class="edit" onclick={
                                                            // TODO: Create a variable for this event
                                                            let popup_box_state = popup_box_state.clone();
                                                            let editing_link_id = editing_link_id.clone();
                                                            move |_| {
                                                                editing_link_id.set(Some(link.id.unwrap()));
                                                                popup_box_state.set(PopupBox::EditLink);

                                                            }
                                                        }>{"Edit"}</button>
                                                        <button
                                                            class="open"
                                                            title={format!("Open in {}", link.browser)}
                                                            onclick={
                                                                let browser = link.browser.clone();
                                                                let url = link.url.clone();
                                                                let display_error_data = display_error_data.clone();
                                                                let popup_box_state = popup_box_state.clone();

                                                                move |_| {
                                                                    open_user_browser(url.clone(), browser.clone(),display_error_data.clone(), popup_box_state.clone());
                                                                }
                                                            }
                                                        >{"Open"}</button>
                                                        <button class="delete" onclick={
                                                            let links = links.clone();
                                                            let link = link.clone();
                                                            let editing_link_id = editing_link_id.clone();
                                                            let popup_box_state = popup_box_state.clone();
                                                            move |_| {
                                                                // Delete this link's EditLink component
                                                                if let Some(id) = *editing_link_id {
                                                                    if id == link.id.unwrap() {
                                                                        popup_box_state.set(PopupBox::None);
                                                                        editing_link_id.set(None);
                                                                    }
                                                                }

                                                                let mut old_links = (*links).clone();

                                                                // remove this link from `old_links`
                                                                old_links.retain(|old_link| old_link != &link);

                                                                links.set(old_links.clone());

                                                                // store the links to the filesystem
                                                                spawn_local(async move {
                                                                    let result = store_data(struct_to_string(&old_links).unwrap())
                                                                        .await
                                                                        .unwrap();

                                                                    // if the result is null, it means success
                                                                    if let Some(error) = result.as_string() {
                                                                        console_error!(error);
                                                                    } else {
                                                                        console_log!("Successfully deleted");
                                                                    }
                                                                });
                                                            }
                                                        }>{"Delete"}</button>
                                                    </div>
                                                </div>
                                            </div>
                                        }

                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    } else {}
                    </>
                }
            }).collect::<Html>()
            }
        </div>
        </>
    }
}

/// Open user's selected browser
fn open_user_browser(
    url: String,
    browser: Browser,
    display_error_data: UseStateHandle<Option<DisplayErrorInnerData>>,
    popup_box_state: UseStateHandle<PopupBox>,
) {
    spawn_local(async move {
        let result = open_browser(&url, struct_to_string(&browser).unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap();

        if let Ok(error_reporter) = string_to_struct::<ErrorReporter>(&result) {
            display_error_data.set(Some(DisplayErrorInnerData {
                class: DisplayErrorClass::Error,
                error_reporter,
                // TODO: reporting options
                options_buttons: None,
                options_message: None,
            }));

            popup_box_state.set(PopupBox::DisplayError);
        } else {
            console_log!("Successfully opened");
        }
    });
}
