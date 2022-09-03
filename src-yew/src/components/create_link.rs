use crate::*;
use itertools::Itertools;

#[function_component(CreateLink)]
pub fn new() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let create_link_state = use_context::<CreateLinkState>().unwrap().0;

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
                } else if let Ok(error) = string_to_struct::<LinkSavingError>(&new_link) {
                    console_error!(format!("Error: {:?}", error));
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
