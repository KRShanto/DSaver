#![allow(unused, dead_code)] // WARNING

use link_types::{Link, LinkSavingError};
use serde_json::from_str as string_to_struct;
use serde_json::to_string as struct_to_string;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_log};
use yew::prelude::*;

#[wasm_bindgen(module = "/assets/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = getData, catch)]
    async fn get_data() -> Result<JsValue, JsValue>; // Vec<Link>, null

    #[wasm_bindgen(js_name = addData, catch)]
    async fn add_data(full_data: String, data: String) -> Result<JsValue, JsValue>; // Vec<Link>, Link

    #[wasm_bindgen(js_name = storeData, catch)]
    async fn store_data(full_data: String) -> Result<JsValue, JsValue>;
}

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    let links = use_state(Vec::new);
    let create_link_state = use_state(|| false);
    let edit_link_state = use_state(|| false);
    let editing_link_position = use_state(|| None);

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

    html! {
        <>
        <button onclick={
            let create_link_state = create_link_state.clone();
            move |_| {
                    create_link_state.set(true);
            }
        }>{"Create a New Link"}</button>

        <ShowLinks
            links={links.clone()}
            edit_link_state={edit_link_state.clone()}
            editing_link_position={editing_link_position.clone()}
        />
        if *create_link_state {
            <CreateLink links={links.clone()} create_link_state={create_link_state}/>
        }
        if *edit_link_state {
            <EditLink
                {links}
                {edit_link_state}
                {editing_link_position}
            />
        }

        </>
    }
}

#[derive(Properties, PartialEq)]
struct ShowLinksProps {
    links: UseStateHandle<Vec<Link>>,
    edit_link_state: UseStateHandle<bool>,
    editing_link_position: UseStateHandle<Option<usize>>,
}

#[function_component(ShowLinks)]
fn show_links(props: &ShowLinksProps) -> Html {
    let links = props.links.clone();
    let edit_link_state = props.edit_link_state.clone();
    let editing_link_position = props.editing_link_position.clone();

    let mut i = 0;

    html! {
        <>
        <div>
            {
            (*links).iter().map(|link| {
                i += 1;

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
                    <button onclick={
                        let edit_link_state = edit_link_state.clone();
                        let editing_link_position = editing_link_position.clone();
                        move |_| {
                            editing_link_position.set(Some(i - 1));
                            edit_link_state.set(true);
                        }
                    }>{"Edit"}</button>
                    <button onclick={
                        let links = links.clone();
                        move |_| {
                            let mut old_links = (*links).clone();
                            old_links.remove(i - 1);

                            links.set(old_links.clone());

                            // store the links to the filesystem
                            spawn_local(async move {
                                let result = store_data(struct_to_string(&old_links).unwrap())
                                    .await
                                    .unwrap();

                                // if the result is null, it means success
                                if let Some(error) = result.as_string() {
                                    console_error!(error);
                                } else {
                                    console_log!("Successfully deleted");
                                }
                            });
                        }
                    }>{"Delete"}</button>
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
    create_link_state: UseStateHandle<bool>,
}

#[function_component(CreateLink)]
fn new(props: &CreateLinkProps) -> Html {
    let links = props.links.clone();
    let create_link_state = props.create_link_state.clone();

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
                id: Uuid::new_v4(),
                url,
                title: title.is_empty().then(|| None).unwrap_or(Some(title)),
                tags: tags.split_whitespace().map(|s| s.to_string()).collect(),
                prirority: prirority.chars().next().unwrap(),
                browser,
                complete: false,
                date: "".to_string(), // TODO
            };

            let links = links.clone();
            let create_link_state = create_link_state.clone();
            spawn_local(async move {
                // hide the component
                create_link_state.set(false);

                let new_link = add_data(
                    struct_to_string(&*links).unwrap(),
                    struct_to_string(&link).unwrap(),
                )
                .await
                .unwrap()
                .as_string()
                .unwrap();

                if let Ok(new_link) = string_to_struct::<Link>(&new_link) {
                    console_log!(format!("We found a new link: {:?}", new_link));
                    let mut old_links = (*links).clone();
                    old_links.push(new_link);

                    links.set(old_links);
                } else if let Ok(error) = string_to_struct::<LinkSavingError>(&new_link) {
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

#[derive(Properties, PartialEq)]
struct EditLinkProps {
    links: UseStateHandle<Vec<Link>>,
    editing_link_position: UseStateHandle<Option<usize>>,
    edit_link_state: UseStateHandle<bool>,
}

#[function_component(EditLink)]
fn editlink(props: &EditLinkProps) -> Html {
    let links = props.links.clone();
    let editing_link = props.links[(*props.editing_link_position).unwrap()].clone();
    let edit_link_state = props.edit_link_state.clone();
    let editing_link_position = props.editing_link_position.clone();

    let url_ref = NodeRef::default();
    let title_ref = NodeRef::default();
    let tags_ref = NodeRef::default();
    let prirority_ref = NodeRef::default();
    let browser_ref = NodeRef::default();

    let onclick = {
        let url_ref = url_ref.clone();
        let title_ref = title_ref.clone();
        let tags_ref = tags_ref.clone();
        let prirority_ref = prirority_ref.clone();
        let browser_ref = browser_ref.clone();
        let editing_link = editing_link.clone();

        move |_| {
            let url = url_ref.cast::<HtmlInputElement>().unwrap().value();
            let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
            let tags = tags_ref.cast::<HtmlInputElement>().unwrap().value();
            let prirority = prirority_ref.cast::<HtmlInputElement>().unwrap().value();
            let browser = browser_ref.cast::<HtmlInputElement>().unwrap().value();

            let new_link = Link {
                id: editing_link.id,
                url,
                title: Some(title),
                tags: tags.split_whitespace().map(|s| s.to_string()).collect(),
                prirority: prirority.chars().next().unwrap(),
                browser,
                complete: editing_link.complete,
                date: editing_link.date.clone(), // TODO
            };

            let links = links.clone();
            let editing_link_position = editing_link_position.clone();
            let edit_link_state = edit_link_state.clone();

            spawn_local(async move {
                let mut old_links = (*links).clone();
                old_links[(*editing_link_position).unwrap()] = new_link;
                links.set(old_links.clone());

                // hide this component
                edit_link_state.set(false);
                editing_link_position.set(None);

                // store the links to the filesystem
                spawn_local(async move {
                    let result = store_data(struct_to_string(&old_links).unwrap())
                        .await
                        .unwrap();

                    // if the result is null, it means success
                    if let Some(error) = result.as_string() {
                        console_error!(error);
                    } else {
                        console_log!("Successfully updated");
                    }
                });
            });
        }
    };
    html! {
        <>
            <input type="text" ref={url_ref.clone()} placeholder="Url of the website" value={editing_link.url.clone()}/>
            <br />
            <input type="text" ref={title_ref.clone()} placeholder="Title of the website" value={editing_link.title.clone()}/>
            <br />
            <input type="text" ref={tags_ref.clone()} placeholder="Tags" value={editing_link.tags.join(" ")}/>
            <br />
            <input type="text" ref={prirority_ref.clone()} placeholder="Prirority" value={editing_link.prirority.to_string()}/>
            <br />
            <input type="text" ref={browser_ref.clone()} placeholder="Browser" value={editing_link.browser.clone()}/>
            <br />

            <button {onclick}>{"Update"}</button>
        </>
    }
}
