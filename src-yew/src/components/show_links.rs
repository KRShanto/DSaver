use crate::*;

#[derive(Properties, PartialEq)]
pub struct ShowLinksProps {
    pub links: UseStateHandle<Vec<Link>>,
    pub edit_link_state: UseStateHandle<bool>,
    pub editing_link_id: UseStateHandle<Option<Uuid>>,
    pub displayed_tags: UseStateHandle<Vec<String>>,
}

#[function_component(ShowLinks)]
pub fn show_links(props: &ShowLinksProps) -> Html {
    let links = props.links.clone();
    let edit_link_state = props.edit_link_state.clone();
    let editing_link_id = props.editing_link_id.clone();
    let displayed_tags = props.displayed_tags.clone();

    let mut displayed_links = Vec::new();

    // looping `links`'s tags with `displayed_tags` and if any `links.tags` match with `displayed_tags`,
    // then the `link` will be pushed into `displayed_links`.
    for display_tag in (*displayed_tags).clone() {
        (*links).iter().for_each(|link| {
            for tag in &link.tags {
                if &display_tag == tag {
                    displayed_links.push(link.clone());
                    break;
                }
            }
        });
    }

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
                        let editing_link_id = editing_link_id.clone();
                        move |_| {
                            let mut old_links = (*links).clone();
                            old_links.remove(
                                old_links
                                    .iter()
                                    .position(|link| link.id == (*editing_link_id).unwrap())
                                    .unwrap(),
                            );

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
