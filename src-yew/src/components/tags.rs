use crate::*;

#[derive(Properties, PartialEq)]
pub struct TagsProps {
    pub links_tags: UseStateHandle<HashMap<String, i32>>,
    pub displayed_tags: UseStateHandle<Vec<String>>,
}

#[function_component(Tags)]
pub fn tags(props: &TagsProps) -> Html {
    let links_tags = props.links_tags.clone();
    let displayed_tags = props.displayed_tags.clone();

    html! {
        <>
        <h1>{"Tags"}</h1>
        <div>
        {
            (*links_tags).iter().map(|(tag, count)| {
                let display = use_state(|| true);

                html! {
                    <p onclick={
                        let tag = tag.clone();
                        let displayed_tags = displayed_tags.clone();
                        let display = display.clone();
                        move |_| {
                            let mut old_displayed_tags = (*displayed_tags).clone();

                            if *display {
                                // remove this tag
                                old_displayed_tags.retain(|old_tag| old_tag != &tag);
                            } else {
                                // push this tag
                                old_displayed_tags.push(tag.clone());
                            }

                            displayed_tags.set(old_displayed_tags);
                            display.set(!*display);
                        }
                    }>{tag}{" - "}{count}</p>
                }
            }).collect::<Html>()
        }
        </div>
        </>
    }
}
