use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
