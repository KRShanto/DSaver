use crate::*;
use itertools::Itertools;

#[function_component(CreateLink)]
pub fn new() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let displayed_tags = use_context::<DisplayedTagsState>().unwrap().0;
    let create_link_state = use_context::<CreateLinkState>().unwrap().0;

    let display_error_state = use_context::<DisplayErrorState>().unwrap().0;
    let display_error_data = use_context::<DisplayErrorData>().unwrap().0;

    let url_value = use_state(String::new);
    let title_value = use_state(String::new);
    let tags_value = use_state(String::new);
    let priority_value = use_state(|| String::from("A"));
    let browser_value = use_state(|| String::from("Default"));

    let title_disabled = use_state(|| true);

    // previously created tags || tags that matches tags from `displayed_tags`
    let previously_matched_tags = use_state(Vec::new);

    let priority_list = (b'A'..=b'Z')
        .map(|c| char::from(c).to_string())
        .collect::<Vec<String>>();

    let onclick = Callback::from({
        let url_value = url_value.clone();
        let title_value = title_value.clone();
        let tags_value = tags_value.clone();
        let priority_value = priority_value.clone();
        let browser_value = browser_value.clone();
        let create_link_state = create_link_state.clone();

        move |_| {
            let url = (*url_value).clone().trim().to_string();
            let title = (*title_value).clone().trim().to_string();
            let tags = (*tags_value).clone().trim().to_string();
            let priority = (*priority_value).clone();
            let browser = browser_value.clone().trim().to_string();
            let display_error_state = display_error_state.clone();
            let display_error_data = display_error_data.clone();

            let link = Link::new_with_date(url)
                .title(title.is_empty().then(|| None).unwrap_or(Some(title)))
                .tags(
                    tags.split_whitespace()
                        .map(|s| s.to_string())
                        .unique()
                        .collect(),
                )
                .priority(priority.parse().unwrap())
                .browser(Browser::from(browser));

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
                } else if let Ok(error) = string_to_struct::<ErrorReporter>(&new_link) {
                    // fill data for `DisplayError` component
                    display_error_data.set(Some(DisplayErrorInnerData {
                        error_reporter: error,
                        options_message: Some(
                            "You can still add the link to the collections. Do you want to add it?"
                                .to_string(),
                        ),
                        options_buttons: Some(vec![
                            (
                                String::from("Add"),
                                Callback::from({
                                    let display_error_state = display_error_state.clone();
                                    let display_error_data = display_error_data.clone();
                                    move |_| {
                                        // push the old (created by user) link to the cold collections
                                        let mut old_links = (*links).clone();
                                        old_links.push(link.clone());

                                        {
                                            let old_links = old_links.clone();
                                            spawn_local(async move {
                                                store_data(struct_to_string(&old_links).unwrap())
                                                    .await
                                                    .unwrap();
                                            });
                                        }

                                        links.set(old_links);

                                        // hide the component
                                        display_error_state.set(false);
                                        display_error_data.set(None);
                                    }
                                }),
                            ),
                            (
                                String::from("Cancel"),
                                Callback::from({
                                    let display_error_data = display_error_data.clone();
                                    let display_error_state = display_error_state.clone();
                                    move |_| {
                                        // hide the component
                                        display_error_state.set(false);
                                        display_error_data.set(None);
                                    }
                                }),
                            ),
                        ]),
                    }));

                    // display the component `DisplayError`
                    display_error_state.set(true);
                } else {
                    console_error!("Neither `Link` nor `ErrorReporter` was found")
                }
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
        <Form title="Create a new link" id="create-link" form_render_state={create_link_state} {onclick}>

            <InputWrapper id="create-url">
                <InputDiv>
                    <Label text="Url of the webpage"></Label>
                    <Input value_state={url_value}/>
                </InputDiv>
            </InputWrapper>

            <InputWrapper id="create-title">
                <InputDiv>
                    <Label text="Title of the webpage"></Label>
                    <Input
                        value_state={title_value.clone()}
                        options={
                            UseInputOptions {
                                disabled: *title_disabled,
                                ..Default::default()
                            }
                        }
                    />
                </InputDiv>
                <Checkbox
                    class="title"
                    label_text="Get the title from the webpage"
                    input_value_is_empty={(*title_value).is_empty()}
                    disabled={title_disabled}
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
                                focus_tag();
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
    }
}
