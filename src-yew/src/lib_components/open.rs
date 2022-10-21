use crate::*;

#[derive(Clone, Properties, PartialEq)]
pub struct OpenProps {
    pub href: String,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub browser: Browser,
    #[prop_or_default]
    pub children: Children,
}

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
                let result = open_browser(href, struct_to_string(&browser).unwrap())
                    .await
                    .unwrap()
                    .as_string()
                    .unwrap();

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
