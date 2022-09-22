use crate::*;

#[function_component(Filter)]
pub fn filter() -> Html {
    let links_tags = use_context::<LinksTagsState>().unwrap().0;
    let links_browsers = use_context::<LinksBrowsersState>().unwrap().0;

    html! {
        <div class="filter">
            if !(*links_tags).is_empty() {
                <Tags />
            }
            if !(*links_browsers).is_empty() {
                <Browsers />
            }
        </div>
    }
}
