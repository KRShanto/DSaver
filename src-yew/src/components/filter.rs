use crate::*;

/// Component for filtering links
///
/// With this component you can filter links by their `tags` and `browser` attributes
///
/// It will display all tags and browser from all links and when you click on them, only that links will be displayed which have the same tag or browser
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
