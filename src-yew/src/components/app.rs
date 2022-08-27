use crate::*;

#[function_component(App)]
pub fn app() -> Html {
    let links = use_state(Vec::new);
    let create_link_state = use_state(|| false);
    let edit_link_state = use_state(|| false);
    let editing_link_id = use_state(|| None);

    let links_tags = use_state(HashMap::new);
    let displayed_tags = use_state(Vec::new);

    let links_browsers = use_state(HashMap::new);
    let displayed_browsers = use_state(Vec::new);

    {
        let links = links.clone();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    // Getting all links from the user's filsystem
                    let data = get_data().await.unwrap().as_string();
                    if let Some(data) = data {
                        // NOTE: The reason the `data` can be `None` is because when an error occurs the function returns `null` instead of String.
                        if let Ok(data) = string_to_struct::<Vec<Link>>(&data) {
                            links.set(data);
                        } else {
                            // Reason: The file's content is not a valid Vec<Link>
                            // TODO: Handle error // Show the user a message that the file is corrupted. And him two options:
                            // 1. Delete the file and start.
                            // 2. Manually fix the file.

                            console_error!("Error: The file is corrupted.");
                        }
                    } else {
                        /* Some reasons why it can be None:
                            1. The file or folder doesn't exist.
                            2. The file is empty.
                        */
                        console_error!("No data found from the filesystem!");
                    }
                });

                || ()
            },
            (),
        );
    }

    {
        let links = links.clone();
        let links_tags = links_tags.clone();
        let displayed_tags = displayed_tags.clone();
        let links_browsers = links_browsers.clone();
        let displayed_browsers = displayed_browsers.clone();
        use_effect_with_deps(
            move |links| {
                let mut tags_map = HashMap::new();
                let mut browsers_map = HashMap::new();

                for link in (**links).clone() {
                    for tag in link.tags.clone() {
                        if let Some(tag) = tags_map.get_mut(&tag) {
                            *tag += 1;
                        } else {
                            tags_map.insert(tag, 1);
                        }
                    }

                    // browsers.push(link.browser.clone());
                    if let Some(browser) = browsers_map.get_mut(&link.browser) {
                        *browser += 1;
                    } else {
                        browsers_map.insert(link.browser, 1);
                    }
                }

                links_tags.set(tags_map.clone());
                displayed_tags.set(tags_map.into_keys().collect::<Vec<String>>());

                links_browsers.set(browsers_map.clone());
                displayed_browsers.set(browsers_map.into_keys().collect::<Vec<String>>());

                || ()
            },
            links,
        );
    }

    html! {
        <>
        <Sidebar {links_tags} create_link_state={create_link_state.clone()} displayed_tags={displayed_tags.clone()} {links_browsers} displayed_browsers={displayed_browsers.clone()}/>

        <ShowLinks
            links={links.clone()}
            edit_link_state={edit_link_state.clone()}
            editing_link_id={editing_link_id.clone()}
            {displayed_tags}
            {displayed_browsers}
        />
        if *create_link_state {
            <CreateLink links={links.clone()} create_link_state={create_link_state}/>
        }
        if *edit_link_state {
            <EditLink
                {links}
                {edit_link_state}
                {editing_link_id}
            />
        }

        </>
    }
}
