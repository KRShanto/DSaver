use crate::*;
use itertools::Itertools;

/// Edit a link
///
/// This component is a popup box and it will appear if you set the [`PopupBox`] to [`PopupBox::EditLink`].
///
/// It will show a form to edit a link. After edit the link, it will save the link to the filesystem and update the [`LinksState`] state.
///
/// Before you can use this component you need to set its data in [`DisplayErrorData`].
///
/// Note that if you remove its data before this component is closed, it will be panic.
///
/// When this component goes out of scope, it will make the state [`DisplayErrorData`] to be [`None`].
///
/// # Warning
///
/// This component will not validate the link. It will just save the link to the filesystem.
#[function_component(EditLink)]
pub fn editlink() -> Html {
    let links = use_context::<LinksState>().unwrap().0;
    let links_tags = use_context::<LinksTagsState>().unwrap().0;
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
    let tags_value = use_state(|| editing_link.tags.join(" "));
    let priority_value = use_state(|| editing_link.priority.to_string());
    let browser_value = use_state(|| editing_link.browser.to_string());

    // previously created tags || tags that matches tags from `displayed_tags`
    let previously_matched_tags = use_state(Vec::new);

    // list of all priorites (from A to Z)
    let priority_list = (b'A'..=b'Z')
        .map(|c| char::from(c).to_string())
        .collect::<Vec<String>>();

    {
        // Remove the id from the `EditingLinkIdState` when the component is unmounted
        let editing_link_id = editing_link_id;
        use_effect_with_deps(move |_| move || editing_link_id.set(None), ());
    }

    let onclick = Callback::from({
        let title = title_value.to_string();
        let description = desc_value.to_string();
        let priority = priority_value.to_string();
        let tags = tags_value.to_string();
        let browser = browser_value.to_string();
        let url = url.clone();

        move |_| {
            // hide the component
            popup_box_state.set(PopupBox::None);
            // TODO: show a loading indicator

            // create a new [`Link`] based on the data from the form
            let new_link = Link {
                id: editing_link.id,
                url: url.clone(),
                title: Some(title.clone()),
                description: Some(description.to_string()),
                tags: tags
                    .split_whitespace()
                    .unique()
                    .map(|s| s.to_string())
                    .collect(),
                priority: priority.chars().next().unwrap(),
                browser: Browser::from(browser.clone()),
                complete: editing_link.complete,
                date: editing_link.date.clone(),
            };

            let links = links.clone();

            spawn_local(async move {
                // change the `links` state with the new link
                let mut old_links = (*links).clone();
                old_links[editing_link_position] = new_link;
                links.set(old_links.clone());

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
        // Whenever the value of `tags_value` field changes, update the `previously_matched_tags` state with the tags that matches the last tag in `tags_value`
        let previously_matched_tags = previously_matched_tags.clone();
        use_effect_with_deps(
            move |tags| {
                let tags = (**tags).to_lowercase();

                // try to get the last tag from the `tags_value`
                match tags.chars().last() {
                    // if the last character is blank, then do not show any tags suggestion
                    Some(tag) => {
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
