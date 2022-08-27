pub use std::collections::HashMap;

pub use link_types::{Link, LinkSavingError};
pub use serde_json::from_str as string_to_struct;
pub use serde_json::to_string as struct_to_string;
pub use uuid::Uuid;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen_futures::spawn_local;
pub use web_sys::HtmlInputElement;
pub use weblog::{console_error, console_log};
pub use yew::prelude::*;

pub use components::*;

pub mod components;

#[wasm_bindgen(module = "/assets/scripts/main.js")]
extern "C" {
    #[wasm_bindgen(js_name = getData, catch)]
    pub async fn get_data() -> Result<JsValue, JsValue>; // Vec<Link>, null

    #[wasm_bindgen(js_name = addData, catch)]
    pub async fn add_data(full_data: String, data: String) -> Result<JsValue, JsValue>; // Vec<Link>, Link

    #[wasm_bindgen(js_name = storeData, catch)]
    pub async fn store_data(full_data: String) -> Result<JsValue, JsValue>;
}
