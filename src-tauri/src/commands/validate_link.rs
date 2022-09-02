use crate::*;
use webpage::{Webpage, WebpageOptions};

/// Validate a link and fetch its title and return it.
#[tauri::command]
pub async fn validate_link(link: String) -> Result<Link, LinkSavingError> {
    // TODO: If the webstie returns 404 error, then warn the user. Create a new variant for LinkSavingError then put the new data in that variant, and if the user confirms that he wants to keep the link, then add that
    let link: Link = serde_json::from_str(&link).unwrap();

    // First check if the url is valid
    if let Ok(req_info) = Webpage::from_url(&link.url, WebpageOptions::default()) {
        let url = url::Url::parse(&req_info.http.url).unwrap();

        let title = if link.title == None {
            req_info.html.title
        } else {
            link.title
        };

        let domain = if let url::Host::Domain(domain) = url.host().unwrap() {
            domain
        } else {
            "" // TODO: provide a better error message
        };

        Ok(Link {
            id: link.id,
            url: link.url,
            title,
            domain: Some(domain.to_string()),
            tags: link.tags,
            browser: link.browser,
            complete: false,
            priority: link.priority,
            date: link.date,
        })
    } else {
        // maybe the website is not working | or the url is not valid
        Err(LinkSavingError::WebpageNotFound)
    }
}
