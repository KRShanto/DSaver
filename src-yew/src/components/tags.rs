use crate::*;

#[function_component(Tags)]
pub fn tags() -> Html {
    let links_tags = use_context::<LinksTagsState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;

    let clicked_tag: UseStateHandle<Option<String>> = use_state(|| None);

    html! {
        <>
        <h1>{"Tags"}</h1>
        <div>
        {
            (*links_tags).iter().map(|(tag, count)| {
                html! {
                    <p onclick={
                        let tag = tag.clone();
                        let displayed_tags = displayed_tags.clone();
                        let clicked_tag = clicked_tag.clone();
                        let links_tags = links_tags.clone();
                        move |_| {
                            // all keys of `links_tags`
                            let mut old_displayed_tags: Vec<String> = (*links_tags).clone().into_keys().collect();

                            // check if the user clicked on the same tag or not
                            if let Some(ctag) = &*clicked_tag {
                                if ctag == &tag {
                                    // user has clicked the same tag. Show all tags (by default)
                                    // change the state to be None
                                    clicked_tag.set(None);
                                } else {
                                    // user has clicked different (this one) tag. Hide other tags
                                    old_displayed_tags.retain(|old_tag| old_tag == &tag);
                                    // change the state to be this tag
                                    clicked_tag.set(Some(tag.clone()));

                                }
                            } else {
                                // user has clicked different (this one) tag. Hide other tags
                                old_displayed_tags.retain(|old_tag| old_tag == &tag);
                                // change the state to be this tag
                                clicked_tag.set(Some(tag.clone()));
                            }

                            // update displayed tags
                            displayed_tags.set(old_displayed_tags);

                        }
                    }>{tag}
                      {" - "}
                      {count}
                      {
                        if let Some(ctag) = &*clicked_tag {
                            if ctag == tag {
                                " - Clicked"
                            } else {
                                ""
                            }
                        } else {
                            ""
                        }
                      }
                    </p>
                }
            }).collect::<Html>()
        }
        </div>
        </>
    }
}
