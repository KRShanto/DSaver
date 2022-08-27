use crate::*;

#[derive(Properties, PartialEq)]
pub struct BrowsersProps {
    pub links_browsers: UseStateHandle<HashMap<String, i32>>,
    pub displayed_browsers: UseStateHandle<Vec<String>>,
}

#[function_component(Browsers)]
pub fn browsers(props: &BrowsersProps) -> Html {
    let links_browsers = props.links_browsers.clone();
    let displayed_browsers = props.displayed_browsers.clone();

    html! {
        <>
        <h1>{"Browsers"}</h1>
        <div>
        {
            (*links_browsers).iter().map(|(browser, count)| {
                let display = use_state(|| true);

                html! {
                    <p onclick={
                        let browser = browser.clone();
                        let displayed_browsers = displayed_browsers.clone();
                        let display = display.clone();
                        move |_| {
                            let mut old_displayed_browsers = (*displayed_browsers).clone();

                            if *display {
                                // remove this browser
                                old_displayed_browsers.retain(|old_browser| old_browser != &browser);
                            } else {
                                // push this browser
                                old_displayed_browsers.push(browser.clone());
                            }

                            displayed_browsers.set(old_displayed_browsers);
                            display.set(!*display);
                        }
                    }>{browser}{" - "}{count}</p>
                }
            }).collect::<Html>()
        }
        </div>
        </>
    }
}
