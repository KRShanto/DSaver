#![allow(unused, dead_code)] // WARNING

// NEXT: Update feature

use link_types::{Link, LinkSavingError};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_log};
use yew::prelude::*;

#[wasm_bindgen(module = "/assets/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = getData, catch)]
    async fn get_data() -> Result<JsValue, JsValue>; // Vec<Link>, null

    #[wasm_bindgen(js_name = storeData, catch)]
    async fn store_data(full_data: String, data: String) -> Result<JsValue, JsValue>; // Vec<Link>, Link
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    let links = use_state(Vec::new);

    {
        let links = links.clone();
        use_effect_with_deps(
            |_| {
                spawn_local(async move {
                    // Getting all links from the user's filsystem
                    let data = get_data().await.unwrap().as_string();
                    if let Some(data) = data {
                        // NOTE: The reason the `data` can be `None` is because when an error occurs the function returns `null` instead of String.
                        if let Ok(data) = serde_json::from_str::<Vec<Link>>(&data) {
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

    html! {
        <>
        <CreateLink links={links.clone()}/>

        <div>
            {
            (*links).iter().map(|link| {
                html! {
                    <>
                    <br />
                    <p>{"URL: "}{&link.url}</p>
                    <p>{format!("Title: {:?}", link.title)}</p>
                    <p>{"Tags: "}</p>
                    <u>
                        {
                            link.tags.iter().map(|tag| {
                                html! {
                                    <li>{tag}</li>
                                }
                            }).collect::<Html>()
                        }
                    </u>
                    <p>{"Priority: "}{link.prirority}</p>
                    <p>{"Browser: "}{&link.browser}</p>
                    <p>{"Complete: "}{link.complete}</p>
                    <p>{"Date: "}{&link.date}</p>
                    </>
                }
            }).collect::<Html>()
            }

        </div>
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct CreateLinkProps {
    links: UseStateHandle<Vec<Link>>,
}

#[function_component(CreateLink)]
fn new(props: &CreateLinkProps) -> Html {
    let links = props.links.clone();

    let url_ref = NodeRef::default();
    let title_ref = NodeRef::default();
    let tags_ref = NodeRef::default();
    let prirority_ref = NodeRef::default();
    let browser_ref = NodeRef::default();

    let title_disabled = use_state(|| true);

    let onclick = {
        let url_ref = url_ref.clone();
        let title_ref = title_ref.clone();
        let tags_ref = tags_ref.clone();
        let prirority_ref = prirority_ref.clone();
        let browser_ref = browser_ref.clone();

        move |_| {
            let url = url_ref.cast::<HtmlInputElement>().unwrap().value();
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            let tags = tags_ref.cast::<HtmlInputElement>().unwrap().value();
            let prirority = prirority_ref.cast::<HtmlInputElement>().unwrap().value();
            let browser = browser_ref.cast::<HtmlInputElement>().unwrap().value();

            let link = Link {
                url,
                title: title.is_empty().then(|| None).unwrap_or(Some(title)),
                tags: tags.split_whitespace().map(|s| s.to_string()).collect(),
                prirority: prirority.chars().next().unwrap(),
                browser,
                complete: false,
                date: "".to_string(), // TODO
            };

            let links = links.clone();
            spawn_local(async move {
                let new_link = store_data(
                    serde_json::to_string(&*links).unwrap(),
                    serde_json::to_string(&link).unwrap(),
                )
                .await
                .unwrap()
                .as_string()
                .unwrap();

                if let Ok(new_link) = serde_json::from_str::<Link>(&new_link) {
                    console_log!(format!("We found a new link: {:?}", new_link));
                    let mut old_links = (*links).clone();
                    old_links.push(new_link);

                    links.set(old_links);
                } else if let Ok(error) = serde_json::from_str::<LinkSavingError>(&new_link) {
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
            <input type="text" ref={prirority_ref.clone()} placeholder="Prirority" value="A"/>
            <br />
            <input type="text" ref={browser_ref.clone()} placeholder="Browser" value="Firefox"/>
            <br />

            <button onclick={onclick}>{"Add"}</button>
        </div>


    }
}
