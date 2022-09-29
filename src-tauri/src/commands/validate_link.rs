use crate::*;
use webpage::{Webpage, WebpageOptions};

/// Validate a link and fetch its title and return it.
#[tauri::command]
pub async fn validate_link(link: String) -> Result<Link, ErrorReporter> {
    // TODO: If the webstie returns 404 error, then warn the user. Create a new variant for LinkSavingError then put the new data in that variant, and if the user confirms that he wants to keep the link, then add that
    let link: Link = serde_json::from_str(&link).unwrap();

    // First check if the url is valid
    match Webpage::from_url(&link.url, WebpageOptions::default()) {
        Ok(req_info) => {
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
                url: req_info.http.url,
                title,
                domain: Some(domain.to_string()),
                tags: link.tags,
                browser: link.browser,
                complete: false,
                priority: link.priority,
                date: link.date,
            })
        }
        Err(err) => {
            // maybe the website is not working | or the url is not valid
            Err(
                ErrorReporterBuilder {
                    error_title: "Not Found",
                    actual_error: &err.to_string(),
                    why_error: vec!["Url is not valid", "The website is not working"],
                    how_to_fix: vec![
                        "Check if you have entered a valid url or not. An example of a valid url is https:www.github.com", 
                        "Make sure the website is working"
                    ],
                    when_error: "creating a new link",
                    error_type: ErrorType::Warning,
                }.build()
            )
        }
    }
}
