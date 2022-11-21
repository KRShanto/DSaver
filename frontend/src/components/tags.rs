use crate::*;

/// Show all tags from the links_state
///
/// If user clicks any of these tag elements, then only those links will be displayed wich link.tags is that clicked tag.
#[function_component(Tags)]
pub fn tags() -> Html {
    let links_tags = use_context::<LinksTagsState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;

    // which browser is selected currently
    let clicked_tag = use_state(|| None);

    html! {
        <div class="filter-tags filterable">
            <h1 class="title">{"Tags"}</h1>
            <div class="tags elements">
            {
                (*links_tags).iter().map(|(tag, count)| {
                    html! {
                        <p class={classes!(
                            "tag",
                            "element",
                            if let Some(ctag) = &*clicked_tag {
                                if ctag == tag {
                                    "clicked"
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            }
                        )} onclick={
                            let tag = tag.clone();
                            let displayed_tags = displayed_tags.clone();
                            let clicked_tag = clicked_tag.clone();
                            let links_tags = links_tags.clone();

                            move |_| {
                                // selected tag
                                // the tag is selected will be store here
                                // after selection this variable will be used to set the state `displayed_tags`
                                // if no tag is selected then all tags will be displayed
                                let mut old_displayed_tags: Vec<String> = (*links_tags).clone().into_keys().collect();

                                // check if the user clicked on the same tag or not
                                if let Some(ctag) = &*clicked_tag {
                                    if ctag == &tag {
                                        // user has clicked the same tag.
                                        // which means that the user wants to unselect the tag
                                        // show all tags (by default)
                                        // change the state to be None
                                        clicked_tag.set(None);
                                    } else {
                                        // user has clicked different (not previously clicked one) tag.
                                        // we need to hide other tag
                                        // remove all tags except the clicked one
                                        old_displayed_tags.retain(|old_tag| old_tag == &tag);
                                        // change the state to be this tag
                                        clicked_tag.set(Some(tag.clone()));
                                    }
                                } else {
                                    // user has clicked this tag
                                    // we need to hide other tags
                                    // remove all tags except the clicked one
                                    old_displayed_tags.retain(|old_tag| old_tag == &tag);
                                    // change the state to be this tag
                                    clicked_tag.set(Some(tag.clone()));
                                }

                                // set the `old_displayed_tags` to the `displayed_tags` state
                                displayed_tags.set(old_displayed_tags);
                            }
                        }>
                            {tag}
                            <span class="count">{count}</span>
                        </p>
                    }
                }).collect::<Html>()
            }
            </div>
        </div>
    }
}
