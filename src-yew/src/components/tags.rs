use crate::*;

#[function_component(Tags)]
pub fn tags() -> Html {
    let links_tags = use_context::<LinksTagsState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;

    let clicked_tag = use_state(|| None);

    // let index = std::rc::Rc::new(std::cell::Cell::new(0));
    let mut index = 0;
    let clicked_index = use_state(|| None);

    html! {
        <div class="filter-tags filterable">
            <h1 class="title">{"Tags"}</h1>
            <div class="tags elements">
            {
                (*links_tags).iter().map(|(tag, count)| {
                    index += 1;

                    html! {
                        <p class={classes!(
                            "tag",
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
                            let tag = tag.clone();
                            let displayed_tags = displayed_tags.clone();
                            let clicked_tag = clicked_tag.clone();
                            let links_tags = links_tags.clone();
                            let clicked_index = clicked_index.clone();

                            move |_| {
                                // all keys of `links_tags`
                                let mut old_displayed_tags: Vec<String> = (*links_tags).clone().into_keys().collect();

                                // check if the user clicked on the same tag or not
                                if let Some(ctag) = &*clicked_tag {
                                    if ctag == &tag {
                                        // user has clicked the same tag. Show all tags (by default)
                                        // change the state to be None
                                        clicked_tag.set(None);
                                        clicked_index.set(None);
                                    } else {
                                        // user has clicked different (this one) tag. Hide other tags
                                        old_displayed_tags.retain(|old_tag| old_tag == &tag);
                                        // change the state to be this tag
                                        clicked_tag.set(Some(tag.clone()));
                                        clicked_index.set(Some(index - 1));
                                    }
                                } else {
                                    // user has clicked different (this one) tag. Hide other tags
                                    old_displayed_tags.retain(|old_tag| old_tag == &tag);
                                    // change the state to be this tag
                                    clicked_tag.set(Some(tag.clone()));
                                    clicked_index.set(Some(index - 1));
                                }

                                // update displayed tags
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
