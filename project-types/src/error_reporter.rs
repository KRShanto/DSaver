use serde::{Deserialize, Serialize};

/// A struct for error handling.
///
/// This struct is used when any error occured in both frontend and backend (backend most likely).
///
/// If any error occurs in the backend/rust side, then you can send the error to the frontend using this struct.
///
/// After sending to the client side, use this struct and show an error message to the user.
///
/// This struct holds lots of information about the error and also how to fix the error.
///
/// Use [`ErrorReporterBuilder`] to instanciate a new [`ErrorReporter`].
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::Read;
/// use home::home_dir;
/// use dsaver_project_types::{ErrorReporter, ErrorReporterBuilder, ErrorType};
///
/// /// Get details about the user
/// fn get_details() -> Result<String, ErrorReporter> {
///     // read the file details.md
///     match home_dir() {
///         Some(home_dir) => {
///             let file_path = home_dir.join("details.md");
///             match File::open(file_path) {
///                 Ok(mut file) => {
///                     // file exists, now read the file and return its content.
///
///                     let mut details = String::new();
///                     file.read_to_string(&mut details).unwrap(); // handle error here
///
///                     Ok(details)
///                 }
///                 Err(err) => Err(ErrorReporterBuilder {
///                     actual_error: &err.to_string(),
///                     why_error: vec![
///                         "The file details.md not found in the home directory",
///                         "Tried to find the file details.md which contains information about you. But the file doesn't exists in the home directory"
///                     ],
///                     how_to_fix: vec![
///                         &format!("Go to your home directory ({})", home_dir.display()),
///                         "Create `details.md` file in your home directory and put some information about you"
///                     ],
///                     error_title: "File Not Found",
///                     when_error: "finding the file details.md in the home directory",
///                     error_type: ErrorType::FileNotFound,
///                 }
///                 .build()),
///             }
///         }
///         None => Err(ErrorReporterBuilder {
///             actual_error: "None",
///             why_error: vec!["Your home directory not found"],
///             how_to_fix: vec!["Put your home directory into the path variable"],
///             error_title: "Directory Not Found",
///             when_error: "finding the home directory",
///             error_type: ErrorType::DirectoryNotFound,
///         }
///         .build()),
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ErrorReporter {
    /// The actual error (i.e. A panic message)
    actual_error: String,
    /// Why the error happened
    why_error: Vec<String>,
    /// How to fix the error
    how_to_fix: Vec<String>,
    /// Title of the error
    error_title: String,
    /// When the error happened
    when_error: String,
    /// Type of the error
    error_type: ErrorType,
}

impl ErrorReporter {
    /// The actual error (i.e. A panic message)
    ///
    /// If you are handling an [`Result`] or anything like that, then you can `match` that result to get the error and put that error into this field.
    ///
    /// This field is for exact error that was encountered.
    ///
    /// If you do not have any such error, then return "None".
    pub fn actual_error(&self) -> &str {
        &self.actual_error
    }

    /// Why the error happened
    ///
    /// You should tell the possible cases where the error might occur.
    ///
    /// Some example reasons:
    ///
    /// - The file *details.md* not found
    ///
    /// - Extected a file named *details.md* in your home directory which will contain some details about you. But found nothing.
    pub fn why_error(&self) -> &Vec<String> {
        &self.why_error
    }

    /// How to fix the error
    ///
    /// You should tell how the user can fix the error or how the user can prevent it in future.
    ///
    /// Some example:
    ///
    /// - Create a file *details.md* in your *home/* directory and write some details about you
    ///
    /// - After creating the file, click the button *Show* again.
    pub fn how_to_fix(&self) -> &Vec<String> {
        &self.how_to_fix
    }

    /// Title of the error
    ///
    /// Some example titles:
    ///
    /// - File not found
    ///
    /// - Invalid username
    pub fn error_title(&self) -> &str {
        &self.error_title
    }

    /// When the error happened
    ///
    /// Describe when the error actually happened.
    ///
    /// *NOTE:*  At the time of [`build`](ErrorReporterBuilder#method.build) this struct, it will automatically put the text `Error occurred when` before this field's value. For example, if you give the text *finding the file*, it will be *Error occured when finding the file* after constructing the struct.
    ///
    /// Some examples:
    ///
    /// - getting the file *details.md*
    pub fn when_error(&self) -> &str {
        &self.when_error
    }

    /// Type of the error
    pub fn error_type(&self) -> &ErrorType {
        &self.error_type
    }
}

/// Builder struct for [`ErrorReporter`]
///
/// After instanciate this struct, call the [`build`](#method.build) method to return a [`ErrorReporter`] instance.
///
/// See the docs of [`ErrorReporter`] for more information.
#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize, PartialEq)]
pub struct ErrorReporterBuilder<'a> {
    /// The actual error (i.e. A panic message)
    ///
    /// If you are handling an [`Result`] or anything like that, then you can `match` that result to get the error and put that error into this field.
    ///
    /// This field is for exact error that was encountered.
    ///
    /// If you do not have any such error, then return "None".
    pub actual_error: &'a str,
    /// Why the error happened
    ///
    /// You should tell the possible cases where the error might occur.
    ///
    /// Some example reasons:
    ///
    /// - The file *details.md* not found
    ///
    /// - Extected a file named *details.md* in your home directory which will contain some details about you. But found nothing.
    pub why_error: Vec<&'a str>,
    /// How to fix the error
    ///
    /// You should tell how the user can fix the error or how the user can prevent it in future.
    ///
    /// Some example:
    ///
    /// - Create a file *details.md* in your *home/* directory and write some details about you
    ///
    /// - After creating the file, click the button *Show* again.
    pub how_to_fix: Vec<&'a str>,
    /// Title of the error
    ///
    /// Some example titles:
    ///
    /// - File not found
    ///
    /// - Invalid username
    pub error_title: &'a str,
    /// When the error happened
    ///
    /// Describe when the error actually happened.
    ///
    /// *NOTE:*  At the time of [`build`](#method.build) this struct, it will automatically put the text `Error occurred when` before this field's value. For example, if you give the text *finding the file*, it will be *Error occured when finding the file* after constructing the struct.
    ///
    /// Some examples:
    ///
    /// - getting the file *details.md*
    pub when_error: &'a str,
    /// Error type
    pub error_type: ErrorType,
}

impl<'a> ErrorReporterBuilder<'a> {
    /// Build a new instance of [`ErrorReporter`].
    pub fn build(&self) -> ErrorReporter {
        ErrorReporter {
            actual_error: self.actual_error.to_string(),
            why_error: self.why_error.iter().map(|s| s.to_string()).collect(),
            how_to_fix: self.how_to_fix.iter().map(|s| s.to_string()).collect(),
            error_title: self.error_title.to_string(),
            when_error: format!(
                "Error occurred when {}",
                self.when_error.to_string().to_lowercase()
            ),
            error_type: self.error_type.clone(),
        }
    }
}

/// A list specifying the types of errors can be occurred from the backend (rust side).
///
/// Different apis will and should add there error types to the list. And that should be documented. It is recommended to mention which variant is for which api or commands.
///
/// They should expose this enum via [`ErrorReporter`] struct. And they should mention in their docs which variants could occur and why.
///
/// This list is intended to grow over time and it is not recommended to exhaustively match against it.
///
/// Users should not focus on every variants if not needed. They should focus on variants which is releted to the apis they are using.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum ErrorType {
    /// The url is invalid or the website is not accessible.
    ///
    /// This variant is from the [`validate_link`](../../../../src-tauri/target/doc/dsaver/fn.validate_link.html) command.
    InvalidOrNotFound,
    /// 404 returned by the website.
    ///
    /// This variant is from the [`validate_link`](../../../../src-tauri/target/doc/dsaver/fn.validate_link.html) command.
    PageNotFound,
    /// Browser not available
    ///
    /// This variant is for [`open_browser_linux`], [`open_browser_macos`] and [`open_browser_windows`] commands.
    ///
    /// [`open_browser_linux`]: ../../../../src-tauri/target/doc/dsaver/fn.open_browser_linux.html
    /// [`open_browser_macos`]: ../../../../src-tauri/target/doc/dsaver/fn.open_browser_macos.html
    /// [`open_browser_windows`]: ../../../../src-tauri/target/doc/dsaver/fn.open_browser_windows.html
    BrowserNotFound,
    /// File not found
    ///
    /// This variant is for every commands who didn't find the expected file
    FileNotFound,
    /// Directory not found
    ///
    /// This variant is for every commands who didn't find the expected directory
    DirectoryNotFound,
    /// Unknown error.
    ///
    /// This variant is for every commands who isn't sure what the error is.
    Others,
}
