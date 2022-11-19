use crate::*;
use itertools::Itertools;

/// Display the links according to their tags and browser fields
///
/// It will display the links that matches the tags in [`DisplayedTagsState`] and the browser field in [`DisplayedBrowsersState`]
///
/// It will display the links in the order of thier priority field.
///
/// When the user double click on a link's *head*, it will open the link in the browser.
///
/// User can also click the arrow buttons to show more options such as edit, delete, and open in browser.
///
/// # Warning
///
/// This component will panic if the `title` and `description` fields (might be more) of the link is `None`.
#[function_component(DisplayLinks)]
pub fn show_links() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let editing_link_id = use_context::<EditingLinkIdState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;
    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    /* How the rendering works here:
        1. Loop all the links through `displayed_tags` state and store all links which's tags are selected to be display (displayed_tags) to the `displayed_links_for_tags` vector.

        2. Loop the browsers through `displayed_browsers` state and then loop the `displayed_links_for_tags` vector and check if the browser of the link is in the `displayed_browsers` state. If it is, then store the link to the `displayed_links_for_browsers` vector.

        3. Store only the unique values of `displayed_links_for_browsers` vector to a new variable `displayed_links`.

        4. Get all the priorities from the `links` state and store them to a new variable `priorities`.
    */

    // all links which's tag is in displayed_tags
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

    // remove duplicate links
    let displayed_links = displayed_links_for_browsers.into_iter().unique();

    // list of priorities of all links
    let mut priorities = Vec::new();

    for link in &*links {
        priorities.push(link.priority);
    }

    // sort the priorities
    priorities.sort();
    // remove duplicate priorities
    priorities.dedup();

    // index of the priority
    let mut priority_index = 0;
    // index of the links to be displayed
    let mut link_index = 0;
    // index of the link which's body (.link-body) is open
    // if this is `None`, then no link is open
    // if this is `Some`, then the link with the index is open
    let opened_link: UseStateHandle<Option<i32>> = use_state(|| None);

    html! {
        <>
        <div class="display-links" id="display-links">
            {
            priorities.iter().map(|priority| {
                priority_index+=1;

                // the links will be shown based on their priority
                // get the links according to the priority
                let links_to_show = displayed_links.clone().into_iter().filter(|link| &link.priority == priority);

                html! {
                    <>
                    if links_to_show.clone().next().is_some() {
                        <div class={classes!(
                            "display-links-container",
                            if priority_index % 2 == 0 {
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
                                        link_index += 1;

                                        html! {
                                            <div class="link">
                                                <div class="link-head">
                                                    <div class="title-area" ondblclick={
                                                        let browser = link.browser.clone();
                                                        let url = link.url.clone();
                                                        let opened_link = opened_link.clone();
                                                        let display_error_data = display_error_data.clone();
                                                        let popup_box_state = popup_box_state.clone();

                                                        move |_| {
                                                            // if current link is not opened, then open the browser on dblclick
                                                            // if current link is opened, then the user can open the link by clicking on the Open button.
                                                            // so no need to open the browser on dblclick
                                                            if let Some(display) = *opened_link {
                                                                if display != (link_index - 1) {
                                                                    open_user_browser(
                                                                        url.clone(),
                                                                        browser.clone(),
                                                                        display_error_data.clone(),
                                                                        popup_box_state.clone(),
                                                                    );
                                                                }
                                                            } else {
                                                                open_user_browser(
                                                                    url.clone(),
                                                                    browser.clone(),
                                                                    display_error_data.clone(),
                                                                    popup_box_state.clone()
                                                                );
                                                            }
                                                        }
                                                    }>
                                                        <h3 class="title">{link.title.clone().unwrap()}</h3>
                                                    </div>
                                                    <div class="icon" onclick={
                                                        let opened_link = opened_link.clone();
                                                        move |_| {
                                                            // toggle the link body
                                                            if let Some(link) = *opened_link {
                                                                if link == (link_index - 1) {
                                                                    // close the link body if user clicked on the opened link
                                                                    opened_link.set(None);
                                                                } else {
                                                                    // open the link body if user clicked on the closed link
                                                                    opened_link.set(Some(link_index - 1));
                                                                }
                                                            } else {
                                                                // open the link body if user clicked on the closed link
                                                                opened_link.set(Some(link_index - 1));
                                                            }

                                                        }
                                                    }>
                                                        if let Some(link) = *opened_link {
                                                            if link == (link_index - 1) {
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
                                                    if let Some(display) = *opened_link {
                                                        if display == (link_index - 1) {
                                                            "display"
                                                        } else {
                                                            ""
                                                        }
                                                    } else {
                                                        ""
                                                    }
                                                )}>
                                                    <div class="info">
                                                        <div class="url-div">
                                                            <p class="url">{&link.url}</p>
                                                            <img
                                                                src="icons/copy.svg"
                                                                alt="Copy"
                                                                title="Copy URL"
                                                                onclick={
                                                                    let url = link.url.clone();
                                                                    move |_| {
                                                                        let url = url.clone();
                                                                        spawn_local(async move {
                                                                            copy_to_clipboard(url).await.unwrap();
                                                                        });
                                                                    }
                                                                }
                                                            />
                                                        </div>

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
                                                        // Open browser button
                                                        <Open
                                                            href={link.url.clone()}
                                                            class="open button"
                                                            browser={link.browser.clone()}
                                                        >{"Open"}</Open>
                                                        // Edit button
                                                        <button class="edit button" onclick={
                                                            let popup_box_state = popup_box_state.clone();
                                                            let editing_link_id = editing_link_id.clone();
                                                            move |_| {
                                                                editing_link_id.set(Some(link.id.unwrap()));
                                                                popup_box_state.set(PopupBox::EditLink);
                                                            }
                                                        }>{"Edit"}</button>
                                                        // Delete button
                                                        <button class="delete button" onclick={
                                                            let links = links.clone();
                                                            let link = link.clone();
                                                            let editing_link_id = editing_link_id.clone();
                                                            let popup_box_state = popup_box_state.clone();
                                                            move |_| {
                                                                // if the link is in editing mode, then close the editing mode else it will cause an error
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

                                                                // update the `links` state without this link
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
        let result = open_browser(url, struct_to_string(&browser).unwrap())
            .await
            .unwrap()
            .as_string()
            .unwrap();

        if let Ok(error_reporter) = string_to_struct::<ErrorReporter>(&result) {
            // fill the `display_error_data` state with the error data
            display_error_data.set(Some(DisplayErrorInnerData {
                class: DisplayErrorClass::Error,
                error_reporter,
                // TODO: reporting options
                options_buttons: None,
                options_message: None,
            }));

            // open the error popup box
            popup_box_state.set(PopupBox::DisplayError);
        } else {
            console_log!("Successfully opened");
        }
    });
}
