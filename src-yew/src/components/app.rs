use crate::*;

#[derive(Clone, PartialEq)]
pub struct LinksState(pub UseStateHandle<Vec<Link>>);
#[derive(Clone, PartialEq)]
pub struct EditingLinkIdState(pub UseStateHandle<Option<Uuid>>);

#[derive(Clone, PartialEq)]
pub struct LinksTagsState(pub UseStateHandle<HashMap<String, i32>>);
#[derive(Clone, PartialEq)]
pub struct DisplayedTagsState(pub UseStateHandle<Vec<String>>);

#[derive(Clone, PartialEq)]
pub struct LinksBrowsersState(pub UseStateHandle<HashMap<Browser, i32>>);
#[derive(Clone, PartialEq)]
pub struct DisplayedBrowsersState(pub UseStateHandle<Vec<Browser>>);

#[derive(Clone, PartialEq)]
pub struct DisplayErrorData(pub UseStateHandle<Option<DisplayErrorInnerData>>);
#[derive(Clone, PartialEq)]
pub struct DisplayErrorInnerData {
    pub error_reporter: ErrorReporter,
    pub options_message: Option<String>,
    pub options_buttons: Option<Vec<(String, Callback<()>)>>,
}

#[derive(Clone, PartialEq)]
pub struct PopupBoxState(pub UseStateHandle<PopupBox>);

#[derive(Clone, PartialEq)]
pub struct PopupBoxReadyState(pub UseStateHandle<bool>);

#[derive(Clone, PartialEq, Eq, Default)]
pub enum PopupBox {
    CreateLink,
    EditLink,
    DisplayError,
    #[default]
    None,
}

#[function_component(App)]
pub fn app() -> Html {
    let editing_link_id = use_state(|| None);

    let links = use_state(Vec::new);

    let links_tags = use_state(HashMap::new);
    let displayed_tags = use_state(Vec::new);

    let links_browsers = use_state(HashMap::new);
    let displayed_browsers = use_state(Vec::new);

    let display_error_data = use_state(|| None);

    let popup_box_state = use_state(PopupBox::default);
    let popup_box_ready_state = use_state(|| false);

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
                            // TODO: Handle error // Show the user a message that the file is corrupted. And him two options:
                            // 1. Delete the file and start.
                            // 2. Manually fix the file.
                            console_error!("Error: The file is corrupted.");
                        }
                    } else {
                        // error only for debugging
                        console_error!("No data found from the filesystem! Create new data");
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

                    if let Some(browser) = browsers_map.get_mut(&link.browser) {
                        *browser += 1;
                    } else {
                        browsers_map.insert(link.browser, 1);
                    }
                }

                links_tags.set(tags_map.clone());
                displayed_tags.set(tags_map.into_keys().collect::<Vec<String>>());

                links_browsers.set(browsers_map.clone());
                displayed_browsers.set(browsers_map.into_keys().collect::<Vec<Browser>>());

                || ()
            },
            links,
        );
    }

    {
        let popup_box_state = popup_box_state.clone();
        use_effect_with_deps(
            move |state| {
                if **state != PopupBox::None {
                    down_opacity("display-links");
                    down_opacity("sidebar");
                } else {
                    up_opacity("display-links");
                    up_opacity("sidebar");
                }

                || ()
            },
            popup_box_state,
        );
    }

    html! {
        <>
        <ContextProvider<LinksState> context={LinksState(links)}>
        <ContextProvider<EditingLinkIdState> context={EditingLinkIdState(editing_link_id)}>
        <ContextProvider<LinksTagsState> context={LinksTagsState(links_tags)}>
        <ContextProvider<DisplayedTagsState> context={DisplayedTagsState(displayed_tags)}>
        <ContextProvider<LinksBrowsersState> context={LinksBrowsersState(links_browsers)}>
        <ContextProvider<DisplayedBrowsersState> context={DisplayedBrowsersState(displayed_browsers)}>
        <ContextProvider<DisplayErrorData> context={DisplayErrorData(display_error_data)}>
        <ContextProvider<PopupBoxState> context={PopupBoxState(popup_box_state.clone())}>
        <ContextProvider<PopupBoxReadyState> context={PopupBoxReadyState(popup_box_ready_state)}>

            <div class="main-div" id="app">
                <Sidebar />
                <DisplayLinks />
            </div>

            {

                match &*popup_box_state {
                    PopupBox::CreateLink => {
                        html! {<CreateLink />}
                    }
                    PopupBox::EditLink => {
                        html! {<EditLink />}
                    }
                    PopupBox::DisplayError => {
                        html! {<DisplayError />}
                    }
                    PopupBox::None => html!{}
                }
            }

        </ContextProvider<PopupBoxReadyState>>
        </ContextProvider<PopupBoxState>>
        </ContextProvider<DisplayErrorData>>
        </ContextProvider<DisplayedBrowsersState>>
        </ContextProvider<LinksBrowsersState>>
        </ContextProvider<DisplayedTagsState>>
        </ContextProvider<LinksTagsState>>
        </ContextProvider<EditingLinkIdState>>
        </ContextProvider<LinksState>>
        </>
    }
}
