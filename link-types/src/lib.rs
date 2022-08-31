use std::fmt::Display;

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

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Browser {
    Firefox,
    Chrome,
    Brave,
    Default,
    // TODO: add more
}

impl Browser {
    pub fn open(&self, path: &str) -> Result<(), std::io::Error> {
        match self {
            Self::Firefox => open::with(path, "firefox"),
            Self::Chrome => open::with(path, "google-chrome"),
            Self::Brave => open::with(path, "brave-browser"),
            Self::Default => open::that(path),
        }
    }
}

impl Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Firefox => f.write_str("Firefox"),
            Self::Chrome => f.write_str("Chrome"),
            Self::Brave => f.write_str("Brave"),
            Self::Default => f.write_str("Default Browser"),
        }
    }
}

impl From<String> for Browser {
    fn from(string: String) -> Self {
        match string.as_str() {
            "Firefox" => Browser::Firefox,
            "Chrome" => Browser::Chrome,
            "Brave" => Browser::Brave,
            _ => Browser::Default,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BrowserOpenError {
    NotFound,
    Other(String),
}
