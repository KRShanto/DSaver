use crate::*;

#[function_component(Browsers)]
pub fn browsers() -> Html {
    let links_browsers = use_context::<LinksBrowsersState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;

    let clicked_browser: UseStateHandle<Option<Browser>> = use_state(|| None);

    html! {
        <>
        <h1>{"Browsers"}</h1>
        <div>
        {
            (*links_browsers).iter().map(|(browser, count)| {
                html! {
                    <p onclick={
                        let browser = browser.clone();
                        let displayed_browsers = displayed_browsers.clone();
                        let clicked_browser = clicked_browser.clone();
                        let links_browsers = links_browsers.clone();
                        move |_| {
                            // all browsers
                            let mut old_displayed_browsers: Vec<Browser> = (*links_browsers).clone().into_keys().collect();

                            // check if the user clicked on the same browser or not
                            if let Some(cbrowser) = &*clicked_browser {
                                if cbrowser == &browser {
                                    // user has clicked the same browser. Show all browsers (by default)
                                    // change the state to be None
                                    clicked_browser.set(None);
                                } else {
                                    // user has clicked different (this on) browser. Hide other browsers
                                    old_displayed_browsers.retain(|old_browser| old_browser == &browser);
                                    // change the state to be this tag
                                    clicked_browser.set(Some(browser.clone()));
                                }
                            } else {
                                // user has clicked different (this on) browser. Hide other browsers
                                old_displayed_browsers.retain(|old_browser| old_browser == &browser);
                                // change the state to be this tag
                                clicked_browser.set(Some(browser.clone()));
                            }

                            displayed_browsers.set(old_displayed_browsers);
                            // display.set(!*display);
                        }
                    }>
                      {browser}
                      {" - "}
                      {count}
                      {
                        if let Some(cbrowser) = &*clicked_browser {
                            if cbrowser == browser {
                                " - Clicked"
                            } else {""}
                        } else {""}
                      }
                    </p>
                }
            }).collect::<Html>()
        }
        </div>
        </>
    }
}
