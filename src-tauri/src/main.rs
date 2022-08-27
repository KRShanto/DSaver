#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use link_types::{Link, LinkSavingError};
use webpage::{Webpage, WebpageOptions};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Validate a link and fetch its title and return it.
#[tauri::command]
async fn validate_link(link: String) -> Result<Link, LinkSavingError> {
    // TODO: If the webstie returns 404 error, then warn the user. Create a new variant for LinkSavingError then put the new data in that variant, and if the user confirms that he wants to keep the link, then add that
    let link: Link = serde_json::from_str(&link).unwrap();

    // First check if the url is valid
    if let Ok(req_info) = Webpage::from_url(&link.url, WebpageOptions::default()) {
        let title = if link.title == None {
            req_info.html.title
        } else {
            link.title
        };

        Ok(Link {
            id: link.id,
            url: link.url,
            title,
            tags: link.tags,
            browser: link.browser,
            complete: false,
            prirority: link.prirority,
            date: link.date,
        })
    } else {
        // maybe the website is not working | or the url is not valid
        Err(LinkSavingError::WebpageNotFound)
    }
}
