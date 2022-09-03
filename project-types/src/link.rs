use crate::Browser;
#[cfg(feature = "wasm")]
use js_sys::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash, Eq)]
pub struct Link {
    pub id: Uuid,
    pub url: String,            // manually
    pub title: Option<String>, // automatically // At the creation of the link, the title be empty.  It will be filled at saving time. And shouldn't be None at get time.
    pub domain: Option<String>, // automatically // filterable // At the creation of the link, the domain be empty. It will be filled at saving time. And shouldn't be None at get time.
    pub tags: Vec<String>,      // manually  // filterable --- keep|remove
    pub priority: char,         // manually  // filterable --- A, B, C...|...C, B, A
    pub browser: Browser,       // manually  // filterable --- keep|remove
    pub complete: bool,         // manually  // filterable --- ture-false|false-true
    pub date: String,           // automatically // filterable --- latest|oldest
}

impl Link {
    pub fn new(url: String) -> Self {
        Link {
            id: Uuid::new_v4(),
            url,
            title: None,
            domain: None,
            tags: Vec::new(),
            priority: 'A',
            browser: Browser::Default,
            complete: false,
            date: String::from(""),
        }
    }

    #[cfg(feature = "wasm")]
    pub fn new_with_date(url: String) -> Self {
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
            id: Uuid::new_v4(),
            url,
            title: None,
            domain: None,
            tags: Vec::new(),
            priority: 'A',
            browser: Browser::Default,
            complete: false,
            date,
        }
    }

    pub fn id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    pub fn domain(mut self, domain: Option<String>) -> Self {
        self.domain = domain;
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn priority(mut self, priority: char) -> Self {
        self.priority = priority;
        self
    }

    pub fn browser(mut self, browser: Browser) -> Self {
        self.browser = browser;
        self
    }

    pub fn complete(mut self, complete: bool) -> Self {
        self.complete = complete;
        self
    }

    pub fn date(mut self, date: String) -> Self {
        self.date = date;
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum LinkSavingError {
    WebpageNotFound,
}
