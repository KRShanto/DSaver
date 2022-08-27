use crate::*;

#[derive(Properties, PartialEq)]
pub struct EditLinkProps {
    pub links: UseStateHandle<Vec<Link>>,
    pub editing_link_id: UseStateHandle<Option<Uuid>>,
    pub edit_link_state: UseStateHandle<bool>,
}

#[function_component(EditLink)]
pub fn editlink(props: &EditLinkProps) -> Html {
    let links = props.links.clone();
    let editing_link_id = props.editing_link_id.clone();
    let editing_link = (links)
        .iter()
        .find(|link| link.id == (*editing_link_id).unwrap())
        .unwrap()
        .clone();
    let editing_link_position = (*links)
        .iter()
        .position(|link| link.id == (*editing_link_id).unwrap())
        .unwrap();
    let edit_link_state = props.edit_link_state.clone();

    let url_ref = NodeRef::default();
    let title_ref = NodeRef::default();
    let tags_ref = NodeRef::default();
    let prirority_ref = NodeRef::default();
    let browser_ref = NodeRef::default();

    let onclick = {
        let url_ref = url_ref.clone();
        let title_ref = title_ref.clone();
        let tags_ref = tags_ref.clone();
        let prirority_ref = prirority_ref.clone();
        let browser_ref = browser_ref.clone();
        let editing_link = editing_link.clone();

        move |_| {
            let url = url_ref.cast::<HtmlInputElement>().unwrap().value();
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            let tags = tags_ref.cast::<HtmlInputElement>().unwrap().value();
            let prirority = prirority_ref.cast::<HtmlInputElement>().unwrap().value();
            let browser = browser_ref.cast::<HtmlInputElement>().unwrap().value();

            let new_link = Link {
                id: editing_link.id,
                url,
                title: Some(title),
                tags: tags.split_whitespace().map(|s| s.to_string()).collect(),
                prirority: prirority.chars().next().unwrap(),
                browser,
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
            <input type="text" ref={prirority_ref.clone()} placeholder="Prirority" value={editing_link.prirority.to_string()}/>
            <br />
            <input type="text" ref={browser_ref.clone()} placeholder="Browser" value={editing_link.browser.clone()}/>
            <br />

            <button {onclick}>{"Update"}</button>
        </>
    }
}
