use crate::*;
use itertools::Itertools;

#[function_component(DisplayLinks)]
pub fn show_links() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let editing_link_id = use_context::<EditingLinkIdState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;

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

    html! {
        <>
        <div class="display-links" id="display-links">
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
                                        html! {
                                            <div class="link">
                                                <div class="info">
                                                    <h3 class="title">{link.title.as_ref().unwrap_or(&String::new())}</h3>
                                                    <p class="url">{&link.url}</p>
                                                    // TODO: copy button
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
                                                        let popup_box_state = popup_box_state.clone();
                                                        let editing_link_id = editing_link_id.clone();
                                                        move |_| {
                                                            editing_link_id.set(Some(link.id));
                                                            popup_box_state.set(PopupBox::EditLink);

                                                        }
                                                    }>{"Edit"}</button>
                                                    <button class="open" onclick={
                                                        // TODO: show a title "open in <browser>"
                                                        let browser = link.browser.clone();
                                                        let path = link.url.clone();

                                                        move |_| {
                                                            let browser = browser.clone();
                                                            let path = path.clone();
                                                            spawn_local(async move {
                                                                let result = open_browser(&path, struct_to_string(&browser).unwrap())
                                                                    .await
                                                                    .unwrap()
                                                                    .as_string()
                                                                    .unwrap();

                                                                if let Ok(error) = string_to_struct::<BrowserOpenError>(&result) {
                                                                match error {
                                                                        BrowserOpenError::NotFound => console_error!("Browser not found"),
                                                                        BrowserOpenError::Other(error) => console_error!(error),
                                                                    }
                                                                } else {
                                                                    console_log!("Successfully opened");
                                                                }
                                                            });
                                                        }
                                                    }>{"Open"}</button>
                                                    <button class="delete" onclick={
                                                        let links = links.clone();
                                                        let link = link.clone();
                                                        let editing_link_id = editing_link_id.clone();
                                                        let popup_box_state = popup_box_state.clone();
                                                        move |_| {
                                                            // Delete this link's EditLink component
                                                            if let Some(id) = *editing_link_id {
                                                                if id == link.id {
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
