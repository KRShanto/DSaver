use crate::*;
use std::fmt::Display;

/// Open the link in the given browser in Windows OS.
///
/// If this function fails to open the browser for any reason, it will give an error inside [`ErrorReporter`] struct.
///
/// You have to make sure that the user is using windows operating system.
///
/// For that you can [`os.platform`] function from tauri api to know which OS is the user using (at runtime).
///
/// # Example
///
/// ```javascript
/// export async function openBrowser() {
///     const path = "github.com";
///     // this variable should be in JSON format
///     const browser = JSON.stringify("Firefox");
///
///     // importing tauri apis
///     const invoke = window.__TAURI__.invoke;
///     const { platform } = window.__TAURI__.os;
///
///     // Get the platform/OS name
///     const platformName = await platform();
///
///     try {
///         // check if the platform is Windows or not
///         if (platformName === "win32") {
///             // call the command
///             await invoke("open_browser_windows", { path, browser });
///         }
///     } catch (err) {
///         console.error("Error occured while opening the browser in your system");
///         console.error(err);
///     }
/// }
/// ```
///
/// [`os.platform`]: https://tauri.app/v1/api/js/os#platform-1
#[tauri::command]
pub async fn open_browser_windows(path: String, browser: String) -> Result<(), ErrorReporter> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();
    let result = browser.open_in_windows(&path);

    handle_browser_open(result, browser)
}

/// Open the link in the given browser in Linux based OS.
///
/// If this function fails to open the browser for any reason, it will give an error inside [`ErrorReporter`] struct.
///
/// You have to make sure that the user is using linux based operating system.
///
/// For that you can [`os.platform`] function from tauri api to know which OS is the user using (at runtime).
///
/// # Example
///
/// ```javascript
/// export async function openBrowser() {
///     const path = "github.com";
///     // this variable should be in JSON format
///     const browser = JSON.stringify("Firefox");
///
///     // importing tauri apis
///     const invoke = window.__TAURI__.invoke;
///     const { platform } = window.__TAURI__.os;
///
///     // Get the platform/OS name
///     const platformName = await platform();
///
///     try {
///         // check if the platform is linux or not
///         if (platformName === "linux") {
///             // call the command
///             await invoke("open_browser_linux", { path, browser });
///         }
///     } catch (err) {
///         console.error("Error occured while opening the browser in your system");
///         console.error(err);
///     }
/// }
/// ```
///
/// [`os.platform`]: https://tauri.app/v1/api/js/os#platform-1
#[tauri::command]
pub async fn open_browser_linux(path: String, browser: String) -> Result<(), ErrorReporter> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();
    let result = browser.open_in_linux(&path);

    handle_browser_open(result, browser)
}

/// Open the link in the given browser in MacOS.
///
/// If this function fails to open the browser for any reason, it will give an error inside [`ErrorReporter`] struct.
///
/// You have to make sure that the user is using mac operating system.
///
/// For that you can [`os.platform`] function from tauri api to know which OS is the user using (at runtime).
///
/// # Example
///
/// ```javascript
/// export async function openBrowser() {
///     const path = "github.com";
///     // this variable should be in JSON format
///     const browser = JSON.stringify("Firefox");
///
///     // importing tauri apis
///     const invoke = window.__TAURI__.invoke;
///     const { platform } = window.__TAURI__.os;
///
///     // Get the platform/OS name
///     const platformName = await platform();
///
///     try {
///         // check if the platform is macos or not
///         if (platformName === "darwin") {
///             // call the command
///             await invoke("open_browser_macos", { path, browser });
///         }
///     } catch (err) {
///         console.error("Error occured while opening the browser in your system");
///         console.error(err);
///     }
/// }
/// ```
///
/// [`os.platform`]: https://tauri.app/v1/api/js/os#platform-1
#[tauri::command]
pub async fn open_browser_macos(path: String, browser: String) -> Result<(), ErrorReporter> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();
    let result = browser.open_in_macos(&path);

    handle_browser_open(result, browser)
}

/// Handle the result of opening the browser
fn handle_browser_open(
    result: Result<(), std::io::Error>,
    browser: Browser,
) -> Result<(), ErrorReporter> {
    match result {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(report_notfound(error.to_string(), &browser)),
            _ => Err(report_others(error.to_string(), &browser)),
        },
    }
}

/// Create a report for not finding a browser
fn report_notfound<S: AsRef<str> + Display>(actual_error: S, browser: &Browser) -> ErrorReporter {
    ErrorReporterBuilder {
        error_title: "Browser Not Found",
        actual_error: actual_error.as_ref(),
        why_error: vec![
            &format!("The selected browser {browser} is not available in your system",),
            &format!("Cannot find the path of {browser}"),
        ],
        how_to_fix: vec![
            &format!("Install {browser}"),
            &format!("Check if the browser {browser} is in your path environment variable"),
        ],
        when_error: &format!("opening the link in {browser}"),
        error_type: ErrorType::BrowserNotFound,
    }
    .build()
}

/// Create a report for unknown errors
fn report_others<S: AsRef<str> + Display>(actual_error: S, browser: &Browser) -> ErrorReporter {
    ErrorReporterBuilder {
        error_title: "Unknown Error Occurred",
        actual_error: actual_error.as_ref(),
        why_error: vec![],
        how_to_fix: vec!["This is unexpected error. So consider reporting this bug"],
        when_error: &format!("opening the link in {browser}"),
        error_type: ErrorType::Others,
    }
    .build()
}
