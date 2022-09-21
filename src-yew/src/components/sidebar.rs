use crate::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let create_link_state = use_context::<CreateLinkState>().unwrap().0;

    html! {
        <div class="sidebar">
            <button class={classes!(
                "create-link-button",
                if *create_link_state {
                    "disabled"
                } else {
                    "active"
                }
            )} onclick={
                let create_link_state = create_link_state.clone();
                move |_| {
                        create_link_state.set(true);
                }
            }>{"Create a New Link"}</button>

            // IN THE FUTURE
            // <p>{"Show full information"}</p>
            // <p>{"Show completed links"}</p>

            <Filter />
        </div>
    }
}
