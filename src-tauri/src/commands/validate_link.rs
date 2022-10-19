use crate::*;
use uuid::Uuid;
use webpage::{Webpage, WebpageOptions};

/// Validate a link and fetch its title and return it.
///
/// This command is useful when you have created a new link from client side and want to validate it or want to get the information about the link.
///
/// It will fetch the title, description and thumbnail for the link. And if they are available then it will return them on a new instance of [`Link`].
///
///
/// *[FUTURE]* If the website returns 404 error, then it will return an error.
///
/// # Arguments
///
/// This function takes an serialized string of [`Link`].
///
/// # Warning
///
/// If the website is not available or the URL is invalid then it will return an error inside [`ErrorReporter`].
///
/// You can show those error massages to the users without modifying the messages or adding additional message.
///
/// You can get the error information by using the `try-catch` statement.
///
/// # Example
///
/// Call this function/command from your javascript file. You can also export this function and call it from rustwasm.
///
/// ```js
/// async function validator() {
///    // tauri api
///    const invoke = window.__TAURI__.invoke;
///
///    // creating a Link
///    // Note that we didn't use `title`, `id`, and `description`. These fields will be added automatically by validating them.
///    const link = {
///        url: "youtubecom",
///        tags: ["Videos", "Music"],
///        priority: 'C',
///        browser: "Firefox",
///        complete: false,
///        date: "14 January 2022"
///    };
///
///    try {
///        // calling the validator command
///        const data = await invoke("validate_link", { link: JSON.stringify(link) });
///
///        console.log("Validation done");
///
///        console.log(`Id of the link: ${data.id}`);
///        console.log(`Url of the link: ${data.url}`);
///        console.log(`Title of the link: ${data.title}`);
///        console.log(`Description of the link: ${data.description}`);
///        console.log("Whole link object: ", data);
///    } catch (err) {
///        // handle any error
///        console.error("Some error occured while validating the link: ", err);
///    }
///}
/// ```
#[tauri::command]
pub async fn validate_link(link: String) -> Result<Link, ErrorReporter> {
    // TODO: If the webstie returns 404 error, then warn the user. Create a new variant for LinkSavingError then put the new data in that variant, and if the user confirms that he wants to keep the link, then add that
    let link: Link = serde_json::from_str(&link).unwrap();

    // First fetch the website and also check if the `url` is valid or not
    match Webpage::from_url(&link.url, WebpageOptions::default()) {
        // Found the website.
        Ok(req_info) => {
            // If the `id` in None, then create a new one
            let id = match link.id {
                Some(id) => Some(id),
                None => Some(Uuid::new_v4()),
            };

            // If the `title` in None, then return the title fetched from the website
            let title = match link.title {
                Some(t) => Some(t),
                None => match req_info.html.title {
                    // Title found from the website
                    Some(t) => Some(t),
                    // Title not found from the website. Return empty string
                    None => Some(String::new()),
                },
            };

            // If the `description` in None, then return the description fetched from the website
            let description = match link.description {
                Some(d) => Some(d),
                None => match req_info.html.description {
                    // Description found from the website
                    Some(d) => Some(d),
                    // Description not found from the website. Return empty string
                    None => Some(String::new()),
                },
            };

            Ok(Link {
                id,
                url: req_info.http.url,
                title,
                description,
                tags: link.tags,
                browser: link.browser,
                complete: link.complete,
                priority: link.priority,
                date: link.date,
            })
        }
        // website not working or URL not valid
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
                    error_type: ErrorType::InvalidOrNotFound,
                }.build()
            )
        }
    }
}
