use crate::*;

#[tauri::command]
pub async fn open_browser(path: String, browser: String) -> Result<(), BrowserOpenError> {
    let browser: Browser = serde_json::from_str(&browser).unwrap();

    match browser.open(&path) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(BrowserOpenError::NotFound),
            _ => Err(BrowserOpenError::Other(error.to_string())),
        },
    }
}
