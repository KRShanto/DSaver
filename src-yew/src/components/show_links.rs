use crate::*;
use itertools::Itertools;

#[derive(Properties, PartialEq)]
pub struct ShowLinksProps {
    pub links: UseStateHandle<Vec<Link>>,
    pub edit_link_state: UseStateHandle<bool>,
    pub editing_link_id: UseStateHandle<Option<Uuid>>,
    pub displayed_tags: UseStateHandle<Vec<String>>,
    pub displayed_browsers: UseStateHandle<Vec<String>>,
}

#[function_component(ShowLinks)]
pub fn show_links(props: &ShowLinksProps) -> Html {
    let links = props.links.clone();
    let edit_link_state = props.edit_link_state.clone();
    let editing_link_id = props.editing_link_id.clone();
    let displayed_tags = props.displayed_tags.clone();
    let displayed_browsers = props.displayed_browsers.clone();

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

    html! {
        <>
        <div>
            {
            displayed_links.into_iter().map(|link| {
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
            }).collect::<Html>()
            }

        </div>
        </>
    }
}
