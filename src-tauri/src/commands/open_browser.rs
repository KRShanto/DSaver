use crate::*;

#[tauri::command]
// TODO try to handle more errors e.g permission errors
// TODO return ErrorReporter
pub async fn open_browser_windows(path: String, browser: String) -> Result<(), BrowserOpenError> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();

    match browser.open_in_windows(&path) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(BrowserOpenError::NotFound),
            _ => Err(BrowserOpenError::Other(error.to_string())),
        },
    }
}

// TODO return ErrorReporter
#[tauri::command]
pub async fn open_browser_linux(path: String, browser: String) -> Result<(), BrowserOpenError> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();

    match browser.open_in_linux(&path) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(BrowserOpenError::NotFound),
            _ => Err(BrowserOpenError::Other(error.to_string())),
        },
    }
}

#[tauri::command]
// TODO return ErrorReporter
pub async fn open_browser_macos(path: String, browser: String) -> Result<(), BrowserOpenError> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();

    match browser.open_in_macos(&path) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(BrowserOpenError::NotFound),
            _ => Err(BrowserOpenError::Other(error.to_string())),
        },
    }
}
