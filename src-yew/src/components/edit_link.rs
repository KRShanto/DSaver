use crate::*;

#[function_component(EditLink)]
pub fn editlink() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let editing_link_id = use_context::<EditingLinkIdState>().unwrap().0;
    let edit_link_state = use_context::<EditLinkState>().unwrap().0;
    let editing_link = (links)
        .iter()
        .find(|link| link.id == (*editing_link_id).unwrap())
        .unwrap()
        .clone();
    let editing_link_position = (*links)
        .iter()
        .position(|link| link.id == (*editing_link_id).unwrap())
        .unwrap();

    let url_ref = NodeRef::default();
    let title_ref = NodeRef::default();
    let tags_ref = NodeRef::default();
    let priority_ref = NodeRef::default();
    let browser_ref = NodeRef::default();

    let onclick = {
        let url_ref = url_ref.clone();
        let title_ref = title_ref.clone();
        let tags_ref = tags_ref.clone();
        let priority_ref = priority_ref.clone();
        let browser_ref = browser_ref.clone();
        let editing_link = editing_link.clone();

        move |_| {
            let url = url_ref.cast::<HtmlInputElement>().unwrap().value();
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            let tags = tags_ref.cast::<HtmlInputElement>().unwrap().value();
            let priority = priority_ref.cast::<HtmlInputElement>().unwrap().value();
            let browser = browser_ref.cast::<HtmlInputElement>().unwrap().value();

            let new_link = Link {
                id: editing_link.id,
                url,
                title: Some(title),
                tags: tags.split_whitespace().map(|s| s.to_string()).collect(),
                priority: priority.chars().next().unwrap(),
                browser: Browser::from(browser),
                complete: editing_link.complete,
                date: editing_link.date.clone(), // TODO
            };

            let links = links.clone();
            let editing_link_id = editing_link_id.clone();
            let edit_link_state = edit_link_state.clone();

            spawn_local(async move {
                let mut old_links = (*links).clone();
                old_links[editing_link_position] = new_link;
                links.set(old_links.clone());

                // hide this component
                edit_link_state.set(false);
                editing_link_id.set(None);

                // store the links to the filesystem
                spawn_local(async move {
                    let result = store_data(struct_to_string(&old_links).unwrap())
                        .await
                        .unwrap();

                    // if the result is null, it means success
                    if let Some(error) = result.as_string() {
                        console_error!(error);
                    } else {
                        console_log!("Successfully updated");
                    }
                });
            });
        }
    };
    html! {
        <>
            <input type="text" ref={url_ref.clone()} placeholder="Url of the website" value={editing_link.url.clone()}/>
            <br />
            <input type="text" ref={title_ref.clone()} placeholder="Title of the website" value={editing_link.title.clone()}/>
            <br />
            <input type="text" ref={tags_ref.clone()} placeholder="Tags" value={editing_link.tags.join(" ")}/>
            <br />
            <input type="text" ref={priority_ref.clone()} placeholder="priority" value={editing_link.priority.to_string()}/>
            <br />
            <input type="text" ref={browser_ref.clone()} placeholder="Browser" value={format!("{}", editing_link.browser)}/>
            <br />

            <button {onclick}>{"Update"}</button>
        </>
    }
}
