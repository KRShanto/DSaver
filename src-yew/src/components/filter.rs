use crate::*;

#[function_component(Filter)]
pub fn filter() -> Html {
    html! {
        <div class="filter">
            <Tags />
            <Browsers />
        </div>
    }
}
