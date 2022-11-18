use crate::*;


/// State of collection of [`Link`]s.
/// 
/// This state will be used everywhere in the app.
/// 
/// So if you need to update/change/delete/add the list of links in frontend, you should update this state.
#[derive(Clone, PartialEq)]
pub struct LinksState(pub UseStateHandle<Vec<Link>>);

/// The `id` of the link who is selected for editing
/// 
/// When the user selects a link to edit, the id of that link will be store in this state so that other components can know that which [`Link`] to edit and display.
/// 
/// If the value is `None`, it means that no link is selected.
#[derive(Clone, PartialEq)]
pub struct EditingLinkIdState(pub UseStateHandle<Option<Uuid>>);


/// List of all tags from all [`Link`]s.
/// 
/// All link's tags will be stored here. And you can use this state everywhere in the app.
/// 
/// The tags will be stored here when the links state [`LinksState`] will be changed.
/// 
/// Before sending this to the context, it will loop through all links and collect all tags and count them and store them in this state.
/// 
/// So you can ensure that when any component is rendering (accept [`App`] component), it will have all tags inside this state.
/// 
/// Inside `UseStateHandle<HashMap<>>`, there are two values (name, how_many_tags)
/// 
/// - First one is the name/value of the tag
/// 
/// - Second one is how many same tags are present
/// 
/// Which means this state won't store the same tags multiple times.
#[derive(Clone, PartialEq)]
pub struct LinksTagsState(pub UseStateHandle<HashMap<String, i32>>);

/// The tags, through which the links will be shown.
/// 
/// Those tags will be stored here which user has selected/filtered to show the links.
/// 
/// This state can be get from anywhere in the app. But it should only be changed when the user selects the tags to show the links associated with those tags.
#[derive(Clone, PartialEq)]
pub struct DisplayedTagsState(pub UseStateHandle<Vec<String>>);

/// List of all browsers from all [`Link`]s.
/// 
/// All link's browsers will be stored here. And you can use this state everywhere in the app.
/// 
/// The browsers will be stored here when the links state [`LinksState`] will be changed.
/// 
/// Before sending this to the context, it will loop through all links and collect all browsers and count them and store them in this state.
/// 
/// So you can ensure that when any component is rendering (accept [`App`] component), it will have all browsers inside this state.
/// 
/// Inside `UseStateHandle<HashMap<>>`, there are two values (name, how_many_browsers)
/// 
/// - First one is the name/value of the browser
/// 
/// - Second one is how many same browsers are present
/// 
/// Which means this state won't store the same browsers multiple times.
#[derive(Clone, PartialEq)]
pub struct LinksBrowsersState(pub UseStateHandle<HashMap<Browser, i32>>);


/// The browsers, through which the links will be shown.
/// 
/// Those browsers will be stored here which user has selected/filtered to show the links associated with those browsers.
/// 
/// This state can be get from anywhere in the app. But it should only be changed when the user selects the browsers to show the links associated with those browsers.
#[derive(Clone, PartialEq)]
pub struct DisplayedBrowsersState(pub UseStateHandle<Vec<Browser>>);

/// Data state for [`DisplayError`] component.
#[derive(Clone, PartialEq)]
pub struct DisplayErrorData(pub UseStateHandle<Option<DisplayErrorInnerData>>);


/// Error data for [`DisplayError`] component.
#[derive(Clone, PartialEq, )]
pub struct DisplayErrorInnerData {
    /// Type of the class
    pub class: DisplayErrorClass,
    /// Reporter data of the error
    pub error_reporter: ErrorReporter,
    /// Options for the user that what he/she can do
    pub options_message: Option<String>,
    /// Options buttons for the user
    pub options_buttons: Option<Vec<DisplayErrorButton>>
}

/// Classes for [`DisplayError`] component.
#[derive(Clone, PartialEq, Eq)]
pub enum DisplayErrorClass {
    /// Warning
    /// 
    /// It will use `warn` as class name
    Warn,
    /// Error
    /// 
    /// It will use `error` as class name
    Error,
}

/// Type of the button for [`DisplayError`] component.
#[derive(Clone, PartialEq, Eq)]
pub enum DisplayErrorButtonType {
    /// Safe
    /// 
    /// It will use blue color for the button
    Safe,
    /// Danger
    /// 
    /// It will use red color for the button
    Danger,
}

/// Button for [`DisplayError`] component.
#[derive(Clone, PartialEq)]
pub struct DisplayErrorButton {
    /// Name of the button
    pub name: String,
    /// Type of the button
    pub button_type: DisplayErrorButtonType,
    /// Callback function for the button
    pub callback: Callback<()>,
}

/// The state of the popup that should be shown to the user.
/// 
/// See [`PopupBox`] enum for more details.
#[derive(Clone, PartialEq)]
pub struct PopupBoxState(pub UseStateHandle<PopupBox>);


/// Is the popup component has rendered?
/// 
/// The [`App`] component listens an onclick event on the `#app` element which detects if the user has clicked outside the popup box.
/// 
/// If the user has clicked outside the popup box, then the popup box will be closed.
/// 
/// But there is a problem. After the user clicks on the button to open the popup box, the popup box will be rendered and the `#app` element will be clicked.
/// 
/// So the popup box will be closed immediately after it has been opened.
/// 
/// To prevent this, we will use this state. When the popup box is rendered, this state will be set to `true`. 
/// 
/// And when the user clicks on the `#app` element, it will check if this state is `true`. If it is `true`, then it will close the popup box.
/// 
/// When the user first clicks on the button to open the popup box, the popup box will not be fully rendered and this state will be false and the `#app` element will be clicked. So the popup box will not be closed. When the popup box is fully rendered, this state will be set to `true` and after that the `#app` element will not be clicked automatically.
/// 
/// So this state will prevent the popup box from closing immediately after it has been opened.
/// 
/// After the popup box is opened, user can close it by clicking on the `#app` element or by clicking on the close button. After that this state will be set to `false`.
#[derive(Clone, PartialEq)]
pub struct PopupBoxReadyState(pub UseStateHandle<bool>);

/// Hide the popup box with animation
/// 
/// You can use this state to hide the popup box with animation. And after the animation is finished, the popup box will be closed.
/// 
/// Or you can use the [`PopupBoxState`] to close the popup box without animation.
#[derive(Clone, PartialEq)]
pub struct PopupBoxHideState(pub UseStateHandle<bool>);

/// Popup that should be shown to the user.
/// 
/// Basically this is an enum that contains popup components.
/// 
/// When you want to show a popup to the user, you should update the [`PopupBoxState`] with this enum.
/// 
/// If you don't need any popup, you can set this state to [`PopupBox::None`].
#[derive(Clone, PartialEq, Eq, Default)]
pub enum PopupBox {
    /// Popup of [`CreateLink`] component.
    /// 
    /// Use it if user wants to create a new link.
    CreateLink,
    /// Popup of [`EditLink`] component.
    /// 
    /// Use it if user wants to edit a link.
    EditLink,
    /// Popup of [`DisplayError`] component.
    /// 
    /// Use it if you want to show an error to the user.
    DisplayError,
    /// No popup.
    /// 
    /// Use it if you don't want to show any popup to the user.
    /// 
    /// This is the default value of this enum.
    #[default]
    None,
    // TODO: Message popup
}

impl DisplayErrorInnerData {
    /// Get a default value for debugging purposes.
    /// 
    /// *It won't work in the production environment. So if you are using this function, make sure to use `cfg(debug_assertions)` to call it only in the dev environment*
    #[cfg(debug_assertions)]
    pub fn get_default(
        popup_box_state: UseStateHandle<PopupBox>, 
        display_error_inner_data: UseStateHandle<Option<Self>>
    ) -> Self {
        let error_reporter = ErrorReporterBuilder {
            actual_error: 
                "thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:4:25 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace",
            why_error: vec!["You have not given a value"],
            how_to_fix: vec!["Give a value to the input", "The value must be valid string"],
            error_title: "Value not found",
            when_error: "getting a input from you",
            error_type: ErrorType::InvalidOrNotFound,
            }.build();

        Self {
            class: DisplayErrorClass::Error,
            error_reporter,
            options_message: Some(String::from("You can again type your input")),
            options_buttons: Some(vec![
                DisplayErrorButton {
                    name: String::from("Type again"),
                    button_type: DisplayErrorButtonType::Safe,
                    callback: Callback::from({
                        move |_| {
                            println!("Type again pressed");

                            popup_box_state.set(PopupBox::None);
                            display_error_inner_data.set(None);
                        }
                    })
                }                         
            ]),
        }
    }
}


/// The main component of the app.
/// 
/// All components will be rendered inside this component.
/// 
/// This component will create some states globally and will pass them to the yew's context api.
/// 
/// So all components can access these states.
#[function_component(App)]
pub fn app() -> Html {
    let editing_link_id = use_state(|| None);

    let links = use_state(Vec::new);

    let links_tags = use_state(HashMap::new);
    let displayed_tags = use_state(Vec::new);

    let links_browsers = use_state(HashMap::new);
    let displayed_browsers = use_state(Vec::new);

    let display_error_data = use_state(|| None);

    let popup_box_state = use_state(|| PopupBox::None);
    let popup_box_ready_state = use_state(|| false);
    let popup_box_hide_state = use_state(|| false);

    // Intial js function to debug/experiment some temporary codes
    #[cfg(debug_assertions)]
    use_effect_with_deps(move |_| {
        spawn_local(async move {
            init().await.unwrap();
        });

        || ()
    }, ());

    { // Get all the links from the filesystem
        let links = links.clone();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    let data = get_data().await.unwrap().as_string();
                    
                    if let Some(data) = data {
                        if let Ok(data) = string_to_struct::<Vec<Link>>(&data) {
                            // data found
                            links.set(data);
                        } else {
                            // TODO: Handle error // Show the user a message that the file is corrupted. And him two options:
                            // 1. Delete the file and start.
                            // 2. Manually fix the file.
                            console_error!("Error: The file is corrupted.");
                        }
                    } else {
                        // TODO: error only for debugging
                        console_error!("No data found from the filesystem! Create new data");
                    }
                });

                || ()
            },
            (),
        );
    }

    
    { // Update the value of `LinksTagsState`, `DisplayedTagsState`, `LinksBrowsersState`, `DisplayedBrowsersState` when the `LinksState` changes
        let links = links.clone();
        let links_tags = links_tags.clone();
        let displayed_tags = displayed_tags.clone();
        let links_browsers = links_browsers.clone();
        let displayed_browsers = displayed_browsers.clone();
        use_effect_with_deps(
            move |links| {
                // list of tags from all links
                // This hashmap will store two things (tag_name, how_many_same_tags)
                // which means it will not store same tags multiple times. Instead it will increment the 2nd value for same tags
                let mut tags_map = HashMap::new();
                // list of browsers from all links
                // This hashmap will store two things (browser_name, how_many_same_browsers)
                // which means it will not store same browsers multiple times. Instead it will increment the 2nd value for same browsers
                let mut browsers_map = HashMap::new();

                // looping over links
                for link in (**links).clone() {
                    // looping over all tags available inside the looped `link`.
                    for tag in link.tags.clone() {
                        // check if the `tag` is present in the `tags_map` or not
                        if let Some(tag) = tags_map.get_mut(&tag) {
                            // If the tag is already present, then we will increment the second value by 1.
                            *tag += 1;
                        } else {
                            // If the tag is not present in the `tags_map`, then we will add it.
                            tags_map.insert(tag, 1);
                        }
                    }

                    // check if the `link.browser` presnet in the `browsers_map` or not
                    if let Some(browser) = browsers_map.get_mut(&link.browser) {
                        // If the browser is already present, then we will increment the second value by 1.
                        *browser += 1;
                    } else {
                        // If the browser is not present in the `browsers_map`, then we will add it
                        browsers_map.insert(link.browser, 1);
                    }
                }

                // update `LinksTagsState with` the new value
                links_tags.set(tags_map.clone());
                // update `DisplayedTagsState` with the new value
                displayed_tags.set(tags_map.into_keys().collect::<Vec<String>>());

                // update `LinksBrowsersState` with the new value
                links_browsers.set(browsers_map.clone());
                // update `DisplayedBrowsersState` with the new value
                displayed_browsers.set(browsers_map.into_keys().collect::<Vec<Browser>>());

                || ()
            },
            links,
        );
    }

    { // When any popup appears, decrease the opacity of the background (#app element) and increase when the popup disappears
        let popup_box_state = popup_box_state.clone();
        use_effect_with_deps(
            move |state| {                
                if **state != PopupBox::None {
                    down_opacity("app");
                } else {
                    up_opacity("app");
                }
                
                || ()
            },
            popup_box_state,
        );
    }
    
    // onclick event for closing the popup box
    // When any popup appears, if the user clicks on the background, the popup will disappear
    // NOTE: This is not the same as clicking on the `X` button    
    let onclick={
        let popup_box_state = popup_box_state.clone();
        let popup_box_ready_state = popup_box_ready_state.clone();
        let popup_box_hide_state = popup_box_hide_state.clone();
        move |_| {
            if *popup_box_ready_state && *popup_box_state != PopupBox::None {
                popup_box_hide_state.set(true);
            }
        }
    };


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
        <ContextProvider<PopupBoxHideState> context={PopupBoxHideState(popup_box_hide_state)}>
            <main class="main-div" id="app" {onclick}>
                <Sidebar />
                <DisplayLinks />
            </main>
                
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

        </ContextProvider<PopupBoxHideState>>
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



