use crate::*;
use itertools::Itertools;

#[function_component(ShowLinks)]
pub fn show_links() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let edit_link_state = use_context::<EditLinkState>().unwrap().0;
    let editing_link_id = use_context::<EditingLinkIdState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;

    // links to display
    let mut displayed_links_for_tags = Vec::new();

    // looping `links`'s tags with `displayed_tags` and if any `links.tags` match with `displayed_tags`,
    // then the `link` will be pushed into `displayed_links`.
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
        prirorities.push(link.prirority);
    }

    prirorities.sort();
    prirorities.dedup();

    html! {
        <>
        <div>
            {
            prirorities.iter().map(|prirority| {
                html! {
                    <>
                    <h1>{prirority}</h1>

                    {
                    displayed_links.clone().into_iter().map(|link| {
                        if &link.prirority == prirority {
                            html! {
                                <>
                                <br />
                                <p>{"URL: "}{&link.url}</p>
                                <p>{format!("Title: {:?}", link.title)}</p>
                                <p>{"Tags: "}</p>
                                <u>
                                {
                                    link.tags.iter().map(|tag| {
                                        html! {
                                                <li>{tag}</li>
                                            }
                                        }).collect::<Html>()
                                    }
                                    </u>
                                <p>{"Priority: "}{link.prirority}</p>
                                <p>{"Browser: "}{&link.browser}</p>
                                <p>{"Complete: "}{link.complete}</p>
                                <p>{"Date: "}{&link.date}</p>
                                <button onclick={
                                    let edit_link_state = edit_link_state.clone();
                                    let editing_link_id = editing_link_id.clone();
                                    move |_| {
                                        editing_link_id.set(Some(link.id));
                                        edit_link_state.set(true);
                                    }
                                }>{"Edit"}</button>
                                <button onclick={
                                    let links = links.clone();
                                    move |_| {
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
                                </>
                            }
                        } else {
                            html!("")
                        }
                    }).collect::<Html>()
                    }
                    </>
                }
            }).collect::<Html>()
            }

        </div>
        </>
    }
}
