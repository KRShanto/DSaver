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
// TODO: Make an example based on examples of ErrorReporterBuilder's fields
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
    /// - Extected a file named *details.md* which will contain some details about you. But found nothing.
    pub fn why_error(&self) -> &Vec<String> {
        &self.why_error
    }

    /// How to fix the error
    ///
    /// You should tell how the user can fix the error or how the user can prevent it in future.
    ///
    /// Some example:
    ///
    /// - Create a file *details.md* in your *home/project/* directory and write some details about you
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
    /// *NOTE:*  At the time of [`build`] this struct, it will automatically put the text `Error occurred when` before this field's value. For example, if you give the text *finding the file*, it will be *Error occured when finding the file* after constructing the struct.
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
/// After instanciate this struct, call the [`build`] method to return a [`ErrorReporter`] instance.
///
/// See the docs of [`ErrorReporter`] for more information.
#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize, PartialEq)]
pub struct ErrorReporterBuilder<'a> {
    /// The actual error (i.e. A panic message)
    ///
    /// If you are handling an [`Result`] or anything like that, then you can `match` that result to get the error and put that error into this field.
    ///
    /// This field is for exact error that was encountered.
    pub actual_error: &'a str,
    /// Why the error happened
    ///
    /// You should tell the possible cases where the error might occur.
    ///
    /// Some example reasons:
    ///
    /// - The file *details.md* not found
    ///
    /// - Extected a file named *details.md* which will contain some details about you. But found nothing.
    pub why_error: Vec<&'a str>,
    /// How to fix the error
    ///
    /// You should tell how the user can fix the error or how the user can prevent it in future.
    ///
    /// Some example:
    ///
    /// - Create a file *details.md* in your *home/project/* directory and write some details about you
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
    /// *NOTE:*  At the time of [`build`] this struct, it will automatically put the text `Error occurred when` before this field's value. For example, if you give the text *finding the file*, it will be *Error occured when finding the file* after constructing the struct.
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
    /// This variant is from the [`validate_link`] command.        
    InvalidOrNotFound,
    /// 404 returned by the website.
    ///
    /// This variant is from the [`validate_link`] command.
    PageNotFound,
}
