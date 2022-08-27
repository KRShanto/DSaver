use crate::*;

#[derive(Properties, PartialEq)]
pub struct FilterProps {
    pub links_tags: UseStateHandle<HashMap<String, i32>>,
    pub displayed_tags: UseStateHandle<Vec<String>>,
}

#[function_component(Filter)]
pub fn filter(props: &FilterProps) -> Html {
    let links_tags = props.links_tags.clone();
    let displayed_tags = props.displayed_tags.clone();

    html! {
        <>
        <Tags {links_tags} {displayed_tags} />
        </>
    }
}
