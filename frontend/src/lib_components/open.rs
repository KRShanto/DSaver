use crate::*;

/// Props for the [`Open`] component.
#[derive(Clone, Properties, PartialEq)]
pub struct OpenProps {
    /// The link to the page.
    ///
    /// This is similar to the `href` attribute in the `<a>` tag.
    pub href: String,
    /// class of the link.
    #[prop_or_default]
    pub class: String,
    /// id of the link.
    #[prop_or_default]
    pub id: String,
    /// In which browser to open the link.
    ///
    /// It is recommended to use the default browser.
    ///
    /// Because the user may not have the browser you specified installed.
    #[prop_or_default]
    pub browser: Browser,
    /// Children of the link.
    ///
    /// It can be a text or any html element.
    #[prop_or_default]
    pub children: Children,
}

/// Open a link in the browser
///
/// In a desktop app, the tag <a> will not open the link in the browser. Instead, it will open the link in the app.
///
/// So, this component will open the link in the browser.
///
/// You can think of this component as an alternative to the <a> tag.
///
/// `props` - [`OpenProps`]
#[function_component(Open)]
pub fn open(props: &OpenProps) -> Html {
    let OpenProps {
        href,
        class,
        id,
        browser,
        children,
    } = (*props).clone();

    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;

    let title = format!("Open in {}", browser);

    let onclick = {
        move |_| {
            let href = href.clone();
            let browser = browser.clone();
            let display_error_data = display_error_data.clone();
            let popup_box_state = popup_box_state.clone();

            spawn_local(async move {
                // Call the rust backend's `open_browser` function to open the link in the browser
                let result = open_browser(href, struct_to_string(&browser).unwrap())
                    .await
                    .unwrap()
                    .as_string()
                    .unwrap();

                // If the result can be parsed as an `ErrorReporter`, then it is an error
                if let Ok(error_reporter) = string_to_struct::<ErrorReporter>(&result) {
                    display_error_data.set(Some(DisplayErrorInnerData {
                        class: DisplayErrorClass::Error,
                        error_reporter,
                        // TODO: reporting options
                        options_buttons: None,
                        options_message: None,
                    }));

                    popup_box_state.set(PopupBox::DisplayError);
                } else {
                    console_log!("Successfully opened");
                }
            });
        }
    };

    html! {
        <div {class} {onclick} {id} {title}>{for children.iter()}</div>
    }
}
