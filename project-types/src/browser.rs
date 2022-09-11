use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Error;
use std::process::Command;

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Browser {
    Firefox,
    Chrome,
    Brave,
    Default,
    // TODO: add more
}

impl Browser {
    /*
    NOTE:
     If the command gives `NotFound`, then the browser might not be available on the system
    */

    /// Get the available browsers
    pub fn get_vec() -> Vec<String> {
        vec![
            String::from("Firefox"),
            String::from("Chrome"),
            String::from("Brave"),
            String::from("Default"),
        ]
    }

    pub fn open_in_windows(&self, url: &str) -> Result<(), Error> {
        let output = if let Some(name) = self.get_browser_name_windows() {
            Command::new("start").args([name, url]).output()
        } else {
            Command::new("start").arg(url).output()
        };

        match output {
            Ok(_) => Ok(()),
            // TODO: give a better error
            Err(err) => Err(err),
        }
    }

    pub fn open_in_linux(&self, url: &str) -> Result<(), Error> {
        let output = if let Some(name) = self.get_browser_name_linux() {
            Command::new(name).arg(url).output()
        } else {
            // Open the default browser
            Command::new("open").arg(url).output()
        };

        match output {
            Ok(_) => Ok(()),
            // TODO: provide a better error
            Err(e) => Err(e),
        }
    }

    // WARNING: This function is not tested
    pub fn open_in_macos(&self, url: &str) -> Result<(), Error> {
        let output = if let Some(name) = self.get_browser_name_macos() {
            Command::new("open").args(["-a", name, url]).output()
        } else {
            Command::new("open").arg(url).output()
        };

        match output {
            Ok(_) => Ok(()),
            // TODO: provide a better error
            Err(e) => Err(e),
        }
    }

    pub fn get_browser_name_windows(&self) -> Option<&str> {
        Some(match self {
            Self::Firefox => "firefox",
            Self::Chrome => "chrome",
            Self::Brave => "brave",
            Self::Default => return None,
        })
    }

    pub fn get_browser_name_linux(&self) -> Option<&str> {
        Some(match self {
            Self::Firefox => "firefox",
            Self::Chrome => "google-chrome",
            Self::Brave => "brave-browser",
            Self::Default => return None,
        })
    }

    pub fn get_browser_name_macos(&self) -> Option<&str> {
        Some(match self {
            Self::Firefox => "Firefox",
            Self::Chrome => "Google Chrome",
            Self::Brave => "Brave Browser",
            Self::Default => return None,
        })
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
            "Firefox" | "firefox" => Browser::Firefox,
            "Chrome" | "chrome" => Browser::Chrome,
            "Brave" | "brave" => Browser::Brave,
            _ => Browser::Default,
        }
    }
}

impl From<&str> for Browser {
    fn from(string: &str) -> Self {
        match string {
            "Firefox" | "firefox" => Browser::Firefox,
            "Chrome" | "chrome" => Browser::Chrome,
            "Brave" | "brave" => Browser::Brave,
            _ => Browser::Default,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BrowserOpenError {
    NotFound,
    Other(String),
}
