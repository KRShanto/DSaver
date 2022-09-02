use crate::Browser;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash, Eq)]
pub struct Link {
    pub id: Uuid,
    pub url: String,           // manually
    pub title: Option<String>, // automatically // At the creation of the link, the title will be empty.  It will be filled at saving time. And shouldn't be None at get time.
    pub tags: Vec<String>,     // manually  // filterable --- keep|remove
    pub priority: char,        // manually  // filterable --- A, B, C...|...C, B, A
    pub browser: Browser,      // manually  // filterable --- keep|remove
    pub complete: bool,        // manually  // filterable --- ture-false|false-true
    pub date: String,          // automatically // filterable --- latest|oldest
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum LinkSavingError {
    WebpageNotFound,
}
