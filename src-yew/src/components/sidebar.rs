use crate::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub create_link_state: UseStateHandle<bool>,
    pub links_tags: UseStateHandle<HashMap<String, i32>>,
    pub displayed_tags: UseStateHandle<Vec<String>>,
    pub links_browsers: UseStateHandle<HashMap<String, i32>>,
    pub displayed_browsers: UseStateHandle<Vec<String>>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let links_tags = props.links_tags.clone();
    let create_link_state = props.create_link_state.clone();
    let displayed_tags = props.displayed_tags.clone();
    let links_browsers = props.links_browsers.clone();
    let displayed_browsers = props.displayed_browsers.clone();

    html! {
        <>
            <button onclick={
                let create_link_state = create_link_state.clone();
                move |_| {
                        create_link_state.set(true);
                }
            }>{"Create a New Link"}</button>

            // IN THE FUTURE
            // <p>{"Show thumbnails"}</p>
            // <p>{"Show descriptions"}</p>
            // <p>{"Show completed links"}</p>

            <Filter {links_tags} {displayed_tags} {links_browsers} {displayed_browsers}/>
        </>
    }
}
