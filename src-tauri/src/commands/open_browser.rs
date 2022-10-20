use std::fmt::Display;

use crate::*;

#[tauri::command]
pub async fn open_browser_windows(path: String, browser: String) -> Result<(), ErrorReporter> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();
    let result = browser.open_in_windows(&path);

    handle_browser_open(result, browser)
}

#[tauri::command]
pub async fn open_browser_linux(path: String, browser: String) -> Result<(), ErrorReporter> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();
    let result = browser.open_in_linux(&path);

    handle_browser_open(result, browser)
}

#[tauri::command]
pub async fn open_browser_macos(path: String, browser: String) -> Result<(), ErrorReporter> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();
    let result = browser.open_in_macos(&path);

    handle_browser_open(result, browser)
}

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
