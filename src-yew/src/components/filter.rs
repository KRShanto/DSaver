use crate::*;

#[function_component(Filter)]
pub fn filter() -> Html {
    html! {
        <>
        <Tags />
        <Browsers />
        </>
    }
}
