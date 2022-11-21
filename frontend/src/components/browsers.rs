use crate::*;

/// Show all browsers from the links_state
///
/// If user clicks any of these browser elements, then only those links will be displayed wich link.browser is that clicked browser.
#[function_component(Browsers)]
pub fn browsers() -> Html {
    let links_browsers = use_context::<LinksBrowsersState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;

    // which browser is selected currently
    let clicked_browser: UseStateHandle<Option<Browser>> = use_state(|| None);

    html! {
        <div class="filter-browsers filterable">
            <h1 class="title">{"Browsers"}</h1>
            <div class="browsers elements">
            {
                (*links_browsers).iter().map(|(browser, count)| {
                    html! {
                        <p class={classes!(
                            "browser",
                            "element",
                            if let Some(cbrowser) = &*clicked_browser {
                                if cbrowser == browser {
                                    "clicked"
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            }
                        )} onclick={
                            let browser = browser.clone();
                            let displayed_browsers = displayed_browsers.clone();
                            let clicked_browser = clicked_browser.clone();
                            let links_browsers = links_browsers.clone();

                            move |_| {
                                // selected browser
                                // the browser is selected will be store here
                                // after selection this variable will be used to set the state `displayed_browsers`
                                // if no browser is selected then all browsers will be displayed
                                let mut old_displayed_browsers: Vec<Browser> = (*links_browsers).clone().into_keys().collect();

                                // check if the user clicked on the same browser or not
                                if let Some(cbrowser) = &*clicked_browser {
                                    if cbrowser == &browser {
                                        // user has clicked the same browser.
                                        // which means that the user wants to unselect the browser
                                        // show all browsers (by default)
                                        // change the state to be None
                                        clicked_browser.set(None);
                                    } else {
                                        // user has clicked different (not previously clicked one) browser.
                                        // we need to hide other browsers
                                        // remove all browsers except the clicked one
                                        old_displayed_browsers.retain(|old_browser| old_browser == &browser);
                                        // change the state to be this tag
                                        clicked_browser.set(Some(browser.clone()));
                                    }
                                } else {
                                    // user has clicked this browser
                                    // we need to hide other browsers
                                    // remove all browsers except the clicked one
                                    old_displayed_browsers.retain(|old_browser| old_browser == &browser);
                                    // change the state to be this tag
                                    clicked_browser.set(Some(browser.clone()));
                                }

                                // set the `old_displayed_browsers` to the `displayed_browsers` state
                                displayed_browsers.set(old_displayed_browsers);
                            }
                        }>
                            {browser}
                            <span class="count">{count}</span>
                        </p>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
