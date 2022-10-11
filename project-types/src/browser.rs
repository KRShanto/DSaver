use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Error;
use std::process::Command;

/// Available browsers through which a link can be opened.
///
/// For opening browsers and links, it runs commands according to the operating system.
///
/// Each browser has each command to open a website.
///
/// For example to open in chrome, you need to type `google-chrome yourwebsite.com` and to open in firefox, you need to type `firefox yourwebsite.com` in linux platforms.
///
/// Thats why opening links in specific browsers has some limitations. For that reason this enum has created.
///
/// User can specify in which browser they want to open a link.
///
/// For example, the user could want to open Music releted videos/links in chrome. And the user could want to open educational videos/links in firefox.
///
/// So he has options to specify which browser he wants to open the link.
///
/// # Example
///
/// You can open a browser using the `open_in_{os}` methods.
///
/// ```ignore
/// # use dsaver_project_types::Browser;
/// #
/// let browser = Browser::Firefox;
///
/// // open in windows
/// browser.open_in_windows("youtube.com");
/// // open in macos
/// browser.open_in_macos("youtube.com");
/// // open in linux platforms
/// browser.open_in_linux("youtube.com");
/// ```
#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum Browser {
    /// Firefox web browser
    Firefox,
    /// Chrome web browser
    Chrome,
    /// Brave web browser
    Brave,
    /// User's default browser
    Default,
    // TODO: add more
}

// implementing Distribution<Browser> for generating random browsers.
impl Distribution<Browser> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Browser {
        match rng.gen_range(0..=2) {
            0 => Browser::Firefox,
            1 => Browser::Chrome,
            2 => Browser::Default,
            _ => Browser::Brave,
        }
    }
}

impl Browser {
    /// Get the available browsers
    pub fn get_vec() -> Vec<String> {
        vec![
            String::from("Default"),
            String::from("Firefox"),
            String::from("Chrome"),
            String::from("Brave"),
        ]
    }

    /// Open the browser in Windows.
    ///
    /// For opening a link in windows, you need to run the command:
    ///
    /// ```shell
    /// cmd \c start {browser} {url}
    /// ```
    ///
    /// This function does the same thing. It runs the command and put the appropriate browser name and URL into the command.
    pub fn open_in_windows(&self, url: &str) -> Result<(), Error> {
        let output = if let Some(name) = self.get_browser_name_windows() {
            Command::new("cmd")
                .args(["/c", "start", name, url])
                .output()
        } else {
            Command::new("cmd").args(["/c", "start", url]).output()
        };

        match output {
            Ok(_) => Ok(()),
            // TODO: give a better error
            Err(err) => Err(err),
        }
    }

    /// Open the browser in linux platforms.
    ///
    /// For opening a link in linux, you need to run the command:
    ///
    /// ```shell
    /// {browser} {url}
    /// ```
    ///
    /// This function does the same thing. it runs the command and put the appropriate browser name and URL into the command.
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

    /// Open the browser in macOS.
    ///
    /// # Warning
    ///
    /// This function is not tested in any macOS platforms. So this function can have some issues.
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

    /// Get the browser name for windows environment for using it in command line for opening that browser.
    pub fn get_browser_name_windows(&self) -> Option<&str> {
        Some(match self {
            Self::Firefox => "firefox",
            Self::Chrome => "chrome",
            Self::Brave => "brave",
            Self::Default => return None,
        })
    }

    /// Get the browser name for linux environment for using it in command line for opening that browser.
    pub fn get_browser_name_linux(&self) -> Option<&str> {
        Some(match self {
            Self::Firefox => "firefox",
            Self::Chrome => "google-chrome",
            Self::Brave => "brave-browser",
            Self::Default => return None,
        })
    }

    /// Get the browser name for macOS environment for using it in command line for opening that browser.
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
    /// Instantiate a new Browser instance from the given `String`.
    ///
    /// You can use both lowercase and uppercase characters.
    ///
    /// For now, you have 3 options: `firefox`, `chrome` and `brave`. If you pass anything else, it will return the `Default` variant.
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Browser;
    /// #
    /// let browser = Browser::from(String::from("firefox"));
    ///
    /// assert_eq!(browser, Browser::Firefox);
    /// ```
    fn from(string: String) -> Self {
        match string.to_lowercase().as_str() {
            "firefox" => Browser::Firefox,
            "chrome" => Browser::Chrome,
            "brave" => Browser::Brave,
            _ => Browser::Default,
        }
    }
}

impl From<&str> for Browser {
    /// Instantiate a new Browser instance from the given `&str`
    ///
    /// You can use both lowercase and uppercase characters.
    ///
    /// For now, you have 3 options: `firefox`, `chrome` and `brave`. If you pass anything else, it will return the `Default` variant.
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Browser;
    /// #
    /// let browser = Browser::from("firefox");
    ///
    /// assert_eq!(browser, Browser::Firefox);
    /// ```
    fn from(string: &str) -> Self {
        match string {
            "Firefox" | "firefox" => Browser::Firefox,
            "Chrome" | "chrome" => Browser::Chrome,
            "Brave" | "brave" => Browser::Brave,
            _ => Browser::Default,
        }
    }
}

/// Possible errors when opening a browser
///
/// # Warning
///
/// In future this enum will be removed. And `ErrorReporter` will be used.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum BrowserOpenError {
    NotFound,
    Other(String),
}
