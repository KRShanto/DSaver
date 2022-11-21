use crate::Browser;
use itertools::Itertools;
#[cfg(feature = "wasm")]
use js_sys::Date;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

/// A struct that represents a webpage link
///
/// This is the main struct for storing the webpage link information.
///
/// For storing a link from client side, you can use [`writeFile`] and [`createDir`] functions from tauri api.
///
/// Before saving the data, you should validate the link/url and then add it to the filesystem
///
/// # Example
///
/// Create a javascript file to call tauri apis
///
/// *file: assets/scripts/write.js*
/// ```js
/// const ROOT_DIR = ".Dsaver";
///
/// export async function writeFile(data) {
///     // importing tauri apis
///     const { writeTextFile, createDir, BaseDirectory } = window.__TAURI__.fs;
///
///     try {
///         // create the directory
///         await createDir(ROOT_DIR, { dir: BaseDirectory.Home, recursive: true });
///         // write the data into the file
///         await writeTextFile(`${ROOT_DIR}/test.json`, data, { dir: BaseDirectory.Home, recursive: true });
///
///         return null;
///     } catch (error) {
///         console.log("ERROR: ", error);
///         return JSON.stringify(error);
///     }
/// }
/// ```
/// And in your rust frontend code call the js function
///
/// ```ignore
/// #[wasm_bindgen(module = "/assets/scripts/write.js")]
/// extern "C" {
///     #[wasm_bindgen(js_name = writeFile, catch)]
///     pub async fn write_file(data: String) -> Result<JsValue, JsValue>;
/// }
/// ```
///
/// Now create a new link and save it to the filesystem
///
/// ```ignore
/// use wasm_bindgen_futures::spawn_local;
/// use wasm_bindgen::prelude::*;
/// use dsaver_project_types::Link;
///
/// let link =
///     Link::new_with_date("https://www.youtube.com/watch?v=ygL_xcavzQ4".to_string())
///         .title("Rust tutorial - Youtube")
///         .priority('C')
///         .tags("Videos Tutorial RustTutorial");
///
/// // save to the home directory
/// spawn_local(async move {
///     write_file(serde_json::to_string(&link).unwrap())
///         .await
///         .unwrap();
/// });
///
/// ```
///
/// [`writeFile`]: https://www.tauri.app/v1/api/js/fs#writefile
/// [`createDir`]: https://www.tauri.app/v1/api/js/fs#createdir
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash, Eq)]
pub struct Link {
    /// Unique identifier for this link
    ///
    /// # Warning
    ///
    /// This shouldn't be `None` when getting it from the filesystem.
    ///
    /// If the value of it is `None`, it means that the link is not saved in the filesystem
    ///
    /// If the value is `Some(Uuid)`, it means that either the link is not saved in the filesystem or the link is saved.
    ///
    /// So before saving it, make sure that it `Some(_)`
    pub id: Option<Uuid>,
    /// HTTP url of the web page
    ///
    /// This must be a valid URL. You should validate the url before saving the link into filesystem.
    pub url: String,
    /// Title of the webpage
    ///
    /// This tag can be automatically fetched from the website by validating the link.
    ///
    /// Or user can use his/her own title.
    ///
    /// # Warning
    ///
    /// It shouldn't be `None` when you are saving the link.
    ///
    /// Otherwise, you will get an error when you try to fetch the link.
    ///
    /// So you should always fetch the title before saving the link.
    ///
    /// If the website doesn't have a title, then it will be empty.
    ///
    /// Note that if the value is `None`, it means that the title has not yet fetched.
    ///
    /// And if the value is Some(""), it means that the the title has already been fetched but the website has empty title.
    ///
    /// And if the value is Some("some title"), it means that the title has already been fetched and the website has a title.
    pub title: Option<String>,
    /// Description of the webapage
    ///
    /// This tag can be automatically fetched from the website by the link.
    ///
    /// Or user can use his/her own description.
    ///
    /// # Warning
    ///
    /// It shouldn't be `None` when you are saving the link.
    ///
    /// Otherwise, you will get an error when you try to fetch the link.
    ///
    /// If the website desn't have a description, then it will be empty.
    ///
    /// Note that if the value is `None`, it means that the description has not yet fetched.
    ///
    /// And if the value is Some(""), it means that the the description has already been fetched but the website has empty title.
    ///
    /// And if the value is Some("some description"), it means that the description has already been fetched and the website has a description.
    pub description: Option<String>,
    /// Tags for the link
    ///
    /// Tag is like a directory where you can structure your multiple files inside a directory. And you go to that specific directory to see only those files that you saved inside that directory.
    ///
    /// With tags, you can use a tag for multiple links, and then you can click on a specific tag and only those links will be shown that uses that tag.
    ///
    /// For example, if you want to save links that are related to python tutorial, then you can create a tag named **PythonTutorial**. And if you want to save links that are related to Funny video, then you can create a tag named **Funny** or **FunnyVideo** and use this tag to all links that are related to funny videos.
    ///
    /// This tag shouldn't be empty. If you the user doesn't give any tag, then the default tag will be `GeneralLink`.
    ///
    /// Tags are very powerful feature of this app.
    ///
    /// *FUTURE* You can hide your links by using tags. You can even lock them with a password.
    ///
    /// *FUTURE* You can use individual passwords for individual tags or you can use a master password for multiple tags.
    pub tags: Vec<String>,
    /// Priority of the link
    ///
    /// The higher the priority, the higher it will appear.
    ///
    /// For example if a link has a priority **A**, and another link has a priority **C**, then the link who has the priority **A** will be appear top of the link that has the priority **C**.
    ///
    /// All priority should be Uppercase character.
    ///
    /// The default value is `A`.
    ///
    /// *FUTURE* You can also filter the links by using the priority.
    pub priority: char,
    /// The browser through which the link will be opened.
    ///
    /// You can also filter the links by using the browser.
    pub browser: Browser,
    /// Is the link is completed or not.
    ///
    /// *FUTURE* You can also filter the links by using this property.
    pub complete: bool,
    /// Link's creation date
    ///
    /// The date will be created when the link is created.
    ///
    /// The format of the date is: `{date} {month} {year}`
    ///
    /// You can use [`new_with_date`](#method.new_with_date) to automatically create a link object with the local date.
    ///
    /// *FUTURE* You can sort links by oldest links or latest links
    pub date: String,
}

impl Link {
    /// Initialize a new [`Link`] with the given `url`
    ///
    /// It will create a link with default values. So keep in mind to update the properties.
    ///
    /// After initializing a new link, you must put a title on the link before saving it to the filesystem.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dsaver_project_types::Browser;
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com".to_string());
    ///
    /// assert_eq!(link.url, "http://example.com".to_string());
    /// assert_eq!(link.title, None);
    /// assert_eq!(link.tags, vec!["GeneralTag".to_string()]);
    /// assert_eq!(link.browser, Browser::SysDefault);
    /// assert_eq!(link.priority, 'A');
    /// assert_eq!(link.complete, false);
    /// assert_eq!(link.date, String::new());    
    /// ```
    pub fn new<T: AsRef<str> + Display>(url: T) -> Self {
        Link {
            id: Some(Uuid::new_v4()),
            url: url.to_string(),
            title: None,
            description: None,
            tags: vec![String::from("GeneralTag")],
            priority: 'A',
            browser: Browser::default(),
            complete: false,
            date: String::from(""),
        }
    }

    /// Initialize a new [`Link`] with the given `url` and with a date (automatically)
    ///
    /// The format of the date is: `{date} {month} {year}`
    ///
    /// This function is intended to be used inside wasm environment because it calls the javascript functions for getting the date
    ///
    /// *You need to enable `wasm` feature to use this method*
    #[cfg(feature = "wasm")]
    pub fn new_with_date<T: AsRef<str> + Display>(url: T) -> Self {
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        let today = Date::new_0();

        let date = format!(
            "{date} {month} {year}",
            date = today.get_date(),
            month = months[today.get_month() as usize],
            year = today.get_full_year()
        );

        Link {
            id: Some(Uuid::new_v4()),
            url: url.to_string(),
            title: None,
            description: None,
            tags: vec![String::from("GeneralTag")],
            priority: 'A',
            browser: Browser::SysDefault,
            complete: false,
            date,
        }
    }

    /// Change the `title` field of the link
    ///
    /// # Example
    ///
    /// ```rust
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the title
    /// assert_eq!(link.title, None);
    ///
    /// // change
    /// let link = link.title("Example website");
    ///
    /// // after changing the title
    /// assert_eq!(link.title, Some("Example website".to_string()));
    /// ```
    pub fn title<T: AsRef<str> + Display>(mut self, title: T) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Change the `description` field of the link
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the description
    /// assert_eq!(link.description, None);
    ///
    /// // change
    /// let link = link.description("An example website. You can find lots of examples in it");
    ///
    /// // after changing the description
    /// assert_eq!(link.description, Some("An example website. You can find lots of examples in it".to_string()));
    /// ```
    pub fn description<T: AsRef<str> + Display>(mut self, description: T) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Change the `tags` field of the link using strings
    ///
    /// The default value is `"GeneralLink"
    ///
    /// Note that each tag should be unique.
    ///
    /// If you use this function to create tags, then it will automatically make the tags unique.
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the tags
    /// assert_eq!(link.tags, vec![String::from("GeneralTag")]);
    ///
    /// // change
    /// let link = link.tags("Videos Tutorial Rust Python");
    ///
    /// // after changing the tags
    /// assert_eq!(link.tags, vec![
    ///     String::from("Videos"),
    ///     String::from("Tutorial"),
    ///     String::from("Rust"),
    ///     String::from("Python"),
    /// ]);
    /// ```
    pub fn tags<T: AsRef<str> + Display>(self, tags: T) -> Self {
        self.tags_vec(tags.to_string().split_whitespace().unique().collect())
    }

    /// Change the `tags` field of the link using Vec of strings
    ///
    /// The default value is `"GeneralLink"
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the tags
    /// assert_eq!(link.tags, vec![String::from("GeneralTag")]);
    ///
    /// // change
    /// let link = link.tags_vec(vec![
    ///     String::from("Videos"),
    ///     String::from("Tutorial"),
    ///     String::from("Rust"),
    ///     String::from("Python"),
    /// ]);
    ///
    /// // after changing the tags
    /// assert_eq!(link.tags, vec![
    ///     String::from("Videos"),
    ///     String::from("Tutorial"),
    ///     String::from("Rust"),
    ///     String::from("Python"),
    /// ]);
    /// ```
    pub fn tags_vec<T: AsRef<str> + Display>(mut self, tags: Vec<T>) -> Self {
        if tags.is_empty() {
            // if the user don't give any value, then the default value
            self.tags = vec![String::from("GeneralLink")];
        } else {
            // else use the user's given value
            self.tags = tags.iter().map(|s| s.to_string()).collect();
        }

        self
    }

    /// Change the `priority` field of the link
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the priority
    /// assert_eq!(link.priority, 'A');
    ///
    /// // change
    /// let link = link.priority('Z');
    ///
    /// // after changing the priority
    /// assert_eq!(link.priority, 'Z');
    /// ```
    pub fn priority(mut self, priority: char) -> Self {
        self.priority = priority;
        self
    }

    /// Change the `browser` field of the link
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// # use dsaver_project_types::Browser;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the browser
    /// assert_eq!(link.browser, Browser::SysDefault);
    ///
    /// // change
    /// let link = link.browser(Browser::Chrome);
    ///
    /// // after changing the browser
    /// assert_eq!(link.browser, Browser::Chrome);
    /// ```
    pub fn browser(mut self, browser: Browser) -> Self {
        self.browser = browser;
        self
    }

    /// change the `complete` field of the link
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the complete
    /// assert_eq!(link.complete, false);
    ///
    /// // change
    /// let link = link.complete(true);
    ///
    /// // after changing the complete
    /// assert_eq!(link.complete, true);
    pub fn complete(mut self, complete: bool) -> Self {
        self.complete = complete;
        self
    }

    /// change the`date` field of the link
    ///
    /// The default value is "" (empty string)
    ///
    /// # Example
    ///
    /// ```
    /// # use dsaver_project_types::Link;
    /// #
    /// let link = Link::new("http://example.com");
    ///
    /// // before changing the date
    /// assert_eq!(link.date, String::new());
    ///
    /// // change
    /// let link = link.date("11 October 2022");
    ///
    /// // after changing the date
    /// assert_eq!(link.date, String::from("11 October 2022"));
    pub fn date<T: AsRef<str> + Display>(mut self, date: T) -> Self {
        self.date = date.to_string();
        self
    }
}
