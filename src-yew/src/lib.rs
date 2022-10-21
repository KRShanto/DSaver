#![allow(clippy::let_unit_value)]

pub use std::collections::HashMap;

pub use dsaver_project_types::*;
pub use js_sys::Function;
pub use serde_json::from_str as string_to_struct;
pub use serde_json::to_string as struct_to_string;
pub use uuid::Uuid;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
pub use wasm_bindgen_futures::spawn_local;
pub use web_sys::HtmlInputElement;
pub use weblog::{console_error, console_log};
pub use webru::{
    callback, callback_with_arg, clear_interval, clear_timeout, set_interval, set_timeout,
};
pub use yew::prelude::*;
pub use yew::{
    html::{Children, ChildrenRenderer, ChildrenWithProps},
    virtual_dom::VChild,
};

pub use components::*;
pub use hooks::*;
pub use lib_components::*;

pub mod components;
pub mod hooks;
pub mod lib_components;

#[wasm_bindgen(module = "/assets/scripts/communicator.js")]
extern "C" {
    /// Get data from user's filesystem.
    ///
    /// After calling `.await.unwrap().as_string()` it will return `Option<String>`. And it can be `None`. It will be `None` if the file is not exits or can't read the file
    ///
    /// If it returns `Some` then you can parse it as a [`Vec<Link`] *if the file contains valid data*.
    #[wasm_bindgen(js_name = getData, catch)]
    pub async fn get_data() -> Result<JsValue, JsValue>; // Vec<Link>, null

    /// Add data in the file system.
    ///
    /// After calling `.await.unwrap().as_string().unwrap()` will return a `String` which can be parse as [`Link`] or [`LinkSavingError`].
    ///
    /// After calling `.as_string()` it will not return `None`.
    ///
    /// The final `String` can be [`Link`] if this successfully adds the data. This link is a new link after validating the old link passed by this function.
    ///
    /// The final `String` can be [`LinkSavingError`] if the Rust backend sends [`Err(LinkSavingError)`];
    ///
    /// You can also use [`store_data`] function to add the data. But if you need to validate and to add `automatic` infos like `title`, status codes, you should use this function because it calls the Rust backend which will fetch informations. *Behind the scene it uses [`store_data`] to store the links.*
    ///
    /// # Arguments
    ///
    /// `full_data` - it is a JSON string which contains [`Vec<Link`]. This is the list of links the user currently have
    ///
    /// `data` - it is a JSON string which contains [`Link`]. This is the new link the user wants to create
    ///
    #[wasm_bindgen(js_name = addData, catch)]
    pub async fn add_data(full_data: String, data: String) -> Result<JsValue, JsValue>; // Vec<Link>, Link

    /// Store data in user's filesystem
    ///
    /// After calling `.await.unwrap().as_string()` it will return `Option<String>`.
    ///
    /// If this function succcessfully stores the data, then it will return `None`.
    ///
    /// If any errror occurs it will return the error inside the `String`.
    #[wasm_bindgen(js_name = storeData, catch)]
    pub async fn store_data(full_data: String) -> Result<JsValue, JsValue>;

    /// Open browser on the specified browser
    ///
    /// After calling let result = `.await.unwrap().as_string().unwrap()`, if it can be parsed as a `BrowserOpenError` then it means an error occurred while opening the browser. Else it means successfully opened the browser.
    ///
    /// The argument `browser` must be a json of `Browser`
    #[wasm_bindgen(js_name = openBrowser, catch)]
    pub async fn open_browser(path: String, browser: String) -> Result<JsValue, JsValue>;

    /// Generate some random links
    ///
    /// This function doesn't return anything
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_name = "generateLink", catch)]
    pub async fn generate_link() -> Result<JsValue, JsValue>;
}

#[wasm_bindgen(module = "/assets/scripts/formStyle.js")]
extern "C" {
    #[wasm_bindgen(js_name = "handleBlurEvent")]
    pub fn handle_blur_event(event: FocusEvent);

    #[wasm_bindgen(js_name = "handleFocusEvent")]
    pub fn handle_focus_event(event: FocusEvent);

    #[wasm_bindgen(js_name = "labelUp")]
    pub fn label_up(input_id: &str);

    #[wasm_bindgen(js_name = "labelDown")]
    pub fn label_down(input_id: &str);

}

#[wasm_bindgen(module = "/assets/scripts/quick.js")]
extern "C" {
    #[wasm_bindgen(js_name = "focusTag")]
    pub fn focus_tag(input_id: &str);

    #[wasm_bindgen(js_name = "ifNotClicked")]
    pub fn if_not_clicked(element_id: &str, what_to_do: &Function);

    #[wasm_bindgen(js_name = "downOpacity")]
    pub fn down_opacity(element_id: &str);

    #[wasm_bindgen(js_name = "upOpacity")]
    pub fn up_opacity(element_id: &str);
}
