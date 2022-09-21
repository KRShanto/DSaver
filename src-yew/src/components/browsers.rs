use crate::*;

#[function_component(Browsers)]
pub fn browsers() -> Html {
    let links_browsers = use_context::<LinksBrowsersState>().unwrap().0;
    let displayed_browsers = use_context::<DisplayedBrowsersState>().unwrap().0;

    let clicked_browser: UseStateHandle<Option<Browser>> = use_state(|| None);

    let mut index = 0;
    let clicked_index = use_state(|| None);

    html! {
        <div class="filter-browsers filterable">
            <h1 class="title">{"Browsers"}</h1>
            <div class="browsers elements">
            {
                (*links_browsers).iter().map(|(browser, count)| {
                    index += 1;

                    html! {
                        <p class={classes!(
                            "browser",
                            "element",
                            if let Some(cindex) = *clicked_index {
                                if cindex == (index - 1) {
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
                            let clicked_index = clicked_index.clone();

                            move |_| {
                                // all browsers
                                let mut old_displayed_browsers: Vec<Browser> = (*links_browsers).clone().into_keys().collect();

                                // check if the user clicked on the same browser or not
                                if let Some(cbrowser) = &*clicked_browser {
                                    if cbrowser == &browser {
                                        // user has clicked the same browser. Show all browsers (by default)
                                        // change the state to be None
                                        clicked_browser.set(None);
                                        clicked_index.set(None);
                                    } else {
                                        // user has clicked different (this on) browser. Hide other browsers
                                        old_displayed_browsers.retain(|old_browser| old_browser == &browser);
                                        // change the state to be this tag
                                        clicked_browser.set(Some(browser.clone()));
                                        // change the clicked index
                                        clicked_index.set(Some(index - 1));
                                    }
                                } else {
                                    // user has clicked different (this on) browser. Hide other browsers
                                    old_displayed_browsers.retain(|old_browser| old_browser == &browser);
                                    // change the state to be this tag
                                    clicked_browser.set(Some(browser.clone()));
                                    // change the clicked index
                                    clicked_index.set(Some(index - 1));
                                }

                                displayed_browsers.set(old_displayed_browsers);
                                // display.set(!*display);
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
