use crate::*;

/// Create a new from the user
///
/// This component is a popup box and it will appear if you set the [`PopupBox`] to [`PopupBox::CreateLink`].
///
/// It will show a form to create a new link. After creating the link, it will validate the link and then add it to the list of links in the user's filesystem.
///
/// It will show an error if the link is not valid or the website is not reachable.
#[function_component(CreateLink)]
pub fn new() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let links_tags = use_context::<LinksTagsState>().unwrap().0;
    let popup_box_state = use_context::<PopupBoxState>().unwrap().0;

    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    let url_value = use_state(String::new);
    let title_value = use_state(String::new);
    let desc_value = use_state(String::new);
    let tags_value = use_state(String::new);
    let priority_value = use_state(|| String::from("A"));
    let browser_value = use_state(|| Browser::default().to_string());

    // is the title field disabled?
    let title_disabled = use_state(|| true);
    // is the description field disabled?
    let desc_disabled = use_state(|| true);

    // previously created tags || tags that matches tags from `links_tags`
    let previously_matched_tags = use_state(Vec::new);

    let priority_list = (b'A'..=b'Z')
        .map(|c| char::from(c).to_string())
        .collect::<Vec<String>>();

    let onclick = Callback::from({
        let url = url_value.trim().to_string();
        let title = title_value.trim().to_string();
        let description = desc_value.trim().to_string();
        let tags = tags_value.to_string();
        let priority = priority_value.to_string();
        let browser = browser_value.to_string();

        move |_| {
            let url = url.clone();
            let title = title.clone();
            let description = description.clone();
            let tags = tags.clone();
            let priority = priority.clone();
            let browser = browser.clone();

            // hide the component
            popup_box_state.set(PopupBox::None);
            // TODO: Show a loading screen

            let display_error_data = display_error_data.clone();

            // Create new Link object
            let link = Link::new_with_date(url)
                .tags(tags)
                .priority(priority.parse().unwrap())
                .browser(Browser::from(browser));

            // If the title is not empty, set it
            // If we set empty string, then we won't get the title from the website
            let link = if title.is_empty() {
                link
            } else {
                link.title(title)
            };

            // If the description is not empty, set it
            // If we set empty string, then we won't get the description from the website
            let link = if description.is_empty() {
                link
            } else {
                link.description(description)
            };

            let links = links.clone();
            let popup_box_state = popup_box_state.clone();

            spawn_local(async move {
                // save the link to the filesystem
                let new_link = add_data(
                    struct_to_string(&*links).unwrap(),
                    struct_to_string(&link).unwrap(),
                )
                .await
                .unwrap()
                .as_string()
                .unwrap();

                // It will give new `Link` object with `title`, `description` fields. Show it in the UI
                if let Ok(new_link) = string_to_struct::<Link>(&new_link) {
                    // debug message
                    console_log!(format!("We found a new link: {:?}", new_link));

                    // create a new variable with the value of both `links` and `new_link`
                    let mut old_links = (*links).clone();
                    old_links.push(new_link);

                    // update the state
                    links.set(old_links);
                } else if let Ok(error) = string_to_struct::<ErrorReporter>(&new_link) {
                    console_error!(format!(
                        "Error occured while adding a new link: {:?}",
                        error
                    ));

                    // fill data for `DisplayError` component
                    display_error_data.set(Some(DisplayErrorInnerData {
                        class: DisplayErrorClass::Error,
                        error_reporter: error,
                        options_message: Some(
                            "You can still add the link to the collections. Do you want to add it?"
                                .to_string(),
                        ),
                        options_buttons: Some(vec![DisplayErrorButton {
                            name: String::from("Add"),
                            button_type: DisplayErrorButtonType::Danger,
                            callback: Callback::from({
                                let popup_box_state = popup_box_state.clone();
                                move |_| {
                                    // setting the `title` and `description` fields to the `link` object with empty values because now we can't fetch them from the website for the error
                                    let link = link
                                        .clone()
                                        .title(String::new())
                                        .description(String::new());

                                    // push the old (newly created by user) `link` to the old collections
                                    let mut old_links = (*links).clone();
                                    old_links.push(link);

                                    {
                                        // Save the link to the filesystem
                                        let old_links = old_links.clone();
                                        spawn_local(async move {
                                            store_data(struct_to_string(&old_links).unwrap())
                                                .await
                                                .unwrap();
                                        });
                                    }

                                    // update the state
                                    links.set(old_links);

                                    // hide the component
                                    // popup_box_hide_state.set(true);
                                    popup_box_state.set(PopupBox::None);
                                }
                            }),
                        }]),
                    }));

                    // display the component `DisplayError`
                    popup_box_state.set(PopupBox::DisplayError);
                } else {
                    console_error!("Neither `Link` nor `ErrorReporter` was found")
                }
            });
        }
    });

    {
        // Whenever the value of `tags_value` field changes, update the `previously_matched_tags` state with the tags that matches the last tag in `tags_value`
        let previously_matched_tags = previously_matched_tags.clone();
        use_effect_with_deps(
            move |tags| {
                let tags = (**tags).to_lowercase();

                // try to get the last tag from the `tags_value`
                match tags.chars().last() {
                    Some(tag) => {
                        // If the last character is a space, then don't show any previous tags
                        if tag.to_string() == " " {
                            previously_matched_tags.set(Vec::new());
                        } else {
                            // get the current word / last word and find which tags are matched.
                            // NOTE: the matched tags are only for current word.
                            let current_word = tags.split_whitespace().last().unwrap_or("");

                            // loop the `links_tags` and check if any tag matches the current word
                            // if any tag matches, then push it to this variable.
                            // After finishing the loop, update the `previously_matched_tags` state with this variable.

                            let mut prev_tags_vec = Vec::new();

                            // loop tags
                            for tag in links_tags.keys() {
                                // if the `tag` contains any word from `current_word`, then push it to the `prev_tags_vec`
                                if tag.to_lowercase().contains(current_word) {
                                    prev_tags_vec.push(tag.to_string());
                                }
                            }

                            // update
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
        <Popup title="Create a new link" id="create-link">
            <Form id="create-link" {onclick} button_text="Add">

                <InputWrapper id="create-url">
                    <InputDiv>
                        <Label text="Url of the webpage"></Label>
                        <Input value_state={url_value} init_focus={true} paste={true}/>
                    </InputDiv>
                </InputWrapper>

                <InputWrapper id="create-title">
                    <InputDiv>
                        <Label text="Title of the webpage"></Label>
                        <Input
                            value_state={title_value.clone()}
                            options={
                                UseInputOptions::permission(
                                    if *title_disabled {
                                        InputPermission::Disabled
                                    } else {
                                        InputPermission::default()
                                })
                            }
                        />
                    </InputDiv>
                    <Checkbox
                        label_text="Get the title from the webpage"
                        input_value_is_empty={(*title_value).is_empty()}
                        disabled={title_disabled}
                    />
                </InputWrapper>

                <InputWrapper id="create-description">
                    <InputDiv>
                        <Label text="Description of the webpage"></Label>
                        <Input
                            value_state={desc_value.clone()}
                            options={
                                UseInputOptions::permission(
                                    if *desc_disabled {
                                        InputPermission::Disabled
                                    } else {
                                        InputPermission::default()
                                })
                            }
                        />
                    </InputDiv>
                    <Checkbox
                        label_text="Get the description from the webpage"
                        input_value_is_empty={(*desc_value).is_empty()}
                        disabled={desc_disabled}
                    />
                </InputWrapper>

                <InputWrapper id="create-tags">
                    <InputDiv>
                        <Label text="Tags">
                            <span>{"(separate with spaces)"}</span>
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
                                    focus_tag("input-create-tags");
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
