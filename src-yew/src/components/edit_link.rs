use crate::*;

#[function_component(EditLink)]
pub fn editlink() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let editing_link_id = use_context::<EditingLinkIdState>().unwrap().0;
    let editing_link = (links)
        .iter()
        .find(|link| link.id.unwrap() == (*editing_link_id).unwrap())
        .unwrap()
        .clone();
    let editing_link_position = (*links)
        .iter()
        .position(|link| link.id.unwrap() == (*editing_link_id).unwrap())
        .unwrap();

    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;

    let url = editing_link.url.clone();
    let title_value = use_state(|| editing_link.title.clone().unwrap());
    let desc_value = use_state(|| editing_link.description.clone().unwrap());
    let tags_value = use_state(String::new); // it will be updated when the component is ready. If we use the initial value, then it throws some errors (I don't know exactly why).
    let priority_value = use_state(|| editing_link.priority.to_string());
    let browser_value = use_state(|| editing_link.browser.to_string());

    {
        let tags_value = tags_value.clone();
        use_effect_with_deps(
            move |_| {
                label_up("input-edit-tags");
                tags_value.set(editing_link.tags.join(" "));

                || ()
            },
            (),
        );
    }

    // previously created tags || tags that matches tags from `displayed_tags`
    let previously_matched_tags = use_state(Vec::new);

    let priority_list = (b'A'..=b'Z')
        .map(|c| char::from(c).to_string())
        .collect::<Vec<String>>();

    let onclick = Callback::from({
        let title = title_value.to_string();
        let description = desc_value.to_string();
        let priority = priority_value.to_string();
        let tags = tags_value.to_string();
        let browser = browser_value.to_string();
        let url = url.clone();

        move |_| {
            let new_link = Link {
                id: editing_link.id,
                url: url.clone(),
                title: Some(title.clone()),
                description: Some(description.to_string()),
                tags: tags.split_whitespace().map(|s| s.to_string()).collect(),
                priority: priority.chars().next().unwrap(),
                browser: Browser::from(browser.clone()),
                complete: editing_link.complete,
                date: editing_link.date.clone(), // TODO
            };

            let links = links.clone();
            let editing_link_id = editing_link_id.clone();
            let popup_box_state = popup_box_state.clone();

            spawn_local(async move {
                let mut old_links = (*links).clone();
                old_links[editing_link_position] = new_link;
                links.set(old_links.clone());

                // hide this component
                popup_box_state.set(PopupBox::None);
                editing_link_id.set(None);

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
    });

    {
        let previously_matched_tags = previously_matched_tags.clone();
        use_effect_with_deps(
            move |tags| {
                let tags = (**tags).to_lowercase();

                match tags.chars().last() {
                    // if the last character is blank, then do not show any tags suggestion
                    Some(tag) => {
                        if tag.to_string() == " " {
                            previously_matched_tags.set(Vec::new());
                        } else {
                            // get the current word / last word and find which tags are matched.
                            // NOTE: the matched tags are only for current word.
                            let current_word = tags.split_whitespace().last().unwrap_or("");

                            let mut prev_tags_vec = Vec::new();

                            // loop tags
                            for dis_tag in &*displayed_tags {
                                if dis_tag.to_lowercase().contains(current_word) {
                                    prev_tags_vec.push(dis_tag.to_string());
                                }
                            }

                            previously_matched_tags.set(prev_tags_vec);
                        }
                    }
                    None => previously_matched_tags.set(Vec::new()),
                }

                || ()
            },
            tags_value.clone(),
        );
    }

    html! {
        <Popup title="Edit your link" id="edit-link">
            <Form  id="edit-link" {onclick} button_text={"Update"}>
                <InputWrapper id="edit-url">
                    <InputDiv>
                        <Label text="Url of the webpage"></Label>
                        <Input
                            init_value={url}
                            options={UseInputOptions::permission(InputPermission::ReadOnly)}
                        />
                    </InputDiv>
                </InputWrapper>

                <InputWrapper id="edit-title">
                    <InputDiv>
                        <Label text="Title of the webpage"/>
                        <Input value_state={title_value} />
                    </InputDiv>
                </InputWrapper>

                <InputWrapper id="edit-description">
                    <InputDiv>
                        <Label text="Description of the webpage" />
                        <Input value_state={desc_value} />
                    </InputDiv>
                </InputWrapper>

                <InputWrapper id="edit-tags">
                    <InputDiv>
                        <Label text="Tags">
                            <span>{" (separate with spaces)"}</span>
                        </Label>
                        <Input value_state={tags_value.clone()} />
                    </InputDiv>

                    <InputTags
                        label_text="Current tags"
                        id="current-tags"
                        tags_values={tags_value.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()}
                    />

                    <InputTags
                        id="previous-tags"
                        label_text="Previous tags"
                        tags_values={(*previously_matched_tags).clone()}
                        tag_type={
                            TagsType::Button(Callback::from(
                                move |args: (MouseEvent, String)| {
                                    let (_, tag) = args;
                                    // tags's value
                                    let previous_tags_value = (*tags_value).clone();

                                    // split the tags by whitespace
                                    let mut previous_tags_value_splitted: Vec<&str> =
                                        previous_tags_value.split_whitespace().collect();

                                    // replace the `tag` with the last element in the previous_tags_value_splitted
                                    previous_tags_value_splitted.pop();
                                    previous_tags_value_splitted.push(&tag);

                                    // set the tags_value to update the tag's value
                                    tags_value.set(previous_tags_value_splitted.join(" "));

                                    // focus on the input
                                    focus_tag("input-edit-tags");
                                }
                            ))
                        }
                    />
                </InputWrapper>

                <Select>
                    <SelectLabel text="Priority of the link" />
                    <Box
                        list={priority_list}
                        class="priority-div"
                        id="priority-div"
                        value_state={priority_value}
                    />
                </Select>

                <Select>
                    <SelectLabel text="From which browser you want to open this link" />
                    <Box
                        list={Browser::get_vec()}
                        class="browser-div"
                        id="browser-div"
                        value_state={browser_value}
                    />
                </Select>
            </Form>
        </Popup>
    }
}
