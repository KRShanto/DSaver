use crate::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputTagsProps {
    #[prop_or_default]
    pub tag_type: TagsType,
    pub tags_values: Vec<String>,
    pub label_text: String,
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TagsType {
    #[default]
    Text,
    Button(Callback<(MouseEvent, String)>),
}

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
                    <div {id}>
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
                                            <button class="tag" onclick={
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
