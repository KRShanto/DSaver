use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Link {
    pub id: uuid::Uuid,        // use uuid
    pub url: String,           // manually
    pub title: Option<String>, // automatically // At the creation of the link, the title will be empty.  It will be filled at saving time. And shouldn't be None at get time.
    pub tags: Vec<String>,     // manually  // filterable --- keep|remove
    pub prirority: char,       // manually  // filterable --- A, B, C...|...C, B, A
    pub browser: String,       // manually  // filterable --- keep|remove
    pub complete: bool,        // manually  // filterable --- ture-false|false-true
    pub date: String,          // automatically // filterable --- latest|oldest
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum LinkSavingError {
    WebpageNotFound,
}
