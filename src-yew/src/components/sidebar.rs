use crate::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    // let create_link_state = use_context::<CreateLinkState>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;

    html! {
        <div class="sidebar" id="sidebar">
            <div class="buttons">
                if cfg!(debug_assertions) {
                    <div class="divider">
                        <button class="generate-link" onclick={
                            |_| {
                                #[cfg(debug_assertions)]
                                spawn_local(async move {
                                    generate_link().await.unwrap();
                                });
                            }
                        }>{"Generate"}</button>
                    </div>
                }

                <div class="divider">
                    <button class={classes!(
                        "create-link-button",
                        if let PopupBox::CreateLink = *popup_box_state {
                            "disabled"
                        } else {
                            "active"
                        }

                    )} onclick={
                        move |_| {
                            popup_box_state.set(PopupBox::CreateLink);
                        }
                    }>{"Create a New Link"}</button>
                </div>
            </div>


            // IN THE FUTURE
            // <p>{"Show full information"}</p>
            // <p>{"Show completed links"}</p>

            // <button>{"Create a hidden link"}</button>
            // <button>{"Create a hidden tag"}</button>
            /*
                Some ideas about how the *hidden links* will work -->

                1. First the user needs to create a new *hidden tag*.
                    For this, user needs to click on the button "Create a hidden tag" and then choose a password for that tag.

                2. Then the user will click on the "Create a hidden link".
                    Then a form will open up and user is required to enter a *hidden tag*.
                    Then he needs to enter the password of the tag.

                3. All hidden tags will appear that the tags section. But no hidden links will appear by default.
                    User needs to *left* click on a hidden tag and give the password and all links will appear.

                4. User needs to *left* click on a hidden tag to hide all hidden links (if opene).

                5. The hidden link's url, title, etc. will be encrypted (with the password given by the user). And the password will be hashed.
            */

            <Filter />
        </div>
    }
}
