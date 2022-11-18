use crate::*;

/// Props for the [`InputTags`] component.
#[derive(Properties, PartialEq, Clone)]
pub struct InputTagsProps {
    /// Type of the tag
    #[prop_or_default]
    pub tag_type: TagsType,
    /// Values of the tags
    ///
    /// It will generate tags (elements) based on the values.
    ///
    /// If the value is empty, then it will generate nothing.
    ///
    /// You can pass a state's value to this prop.
    pub tags_values: Vec<String>,
    /// Title/label of the tags
    pub label_text: String,
    /// Id of the element
    pub id: String,
}

/// Types of tags
#[derive(Clone, Debug, Default, PartialEq)]
pub enum TagsType {
    /// Text tags
    ///
    /// It will only show the text.
    #[default]
    Text,
    /// Button tags
    ///
    /// When the user clicks on the tag, it will call the callback function.
    ///
    /// This variant holds a callback function that will be called when the user clicks on the tag.
    ///
    /// The callback returns the mouse event and the name of the tag.
    Button(Callback<(MouseEvent, String)>), // (event, tag_name)
}

/// InputTags component
///
/// It is a child component of [`InputWrapper`].
///
/// It makes a button or text which looks like a tag for you.
///
/// See the [`Form`] component for more information.
///
/// `props` - [`InputTagsProps`]
#[function_component(InputTags)]
pub fn input_tags(props: &InputTagsProps) -> Html {
    let InputTagsProps {
        id,
        tag_type,
        tags_values,
        label_text,
    } = (*props).clone();

    html! {
        <>
        if tags_values.is_empty() {
            <></>
        } else {
            <div {id} class="tags-wrapper">
                <p class="title">{label_text}</p>
                {
                    tags_values.into_iter().map(move |tag| {
                        match tag_type.clone() {
                            TagsType::Text => {
                                html! {
                                    <span class="tag">{tag}</span>
                                }
                            }
                            TagsType::Button(onclick) => {
                                html! {
                                    <button class="tag button-tag" onclick={
                                        let tag = tag.clone();
                                        move |event| {
                                            onclick.emit((event, tag.clone()))
                                        }
                                    }>{tag}</button>
                                }
                            }
                        }
                    }).collect::<Html>()
                }
            </div>
        }
        </>
    }
}
