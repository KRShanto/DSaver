use serde::{Deserialize, Serialize};

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
    pub fn actual_error(&self) -> &str {
        &self.actual_error
    }
    pub fn why_error(&self) -> &Vec<String> {
        &self.why_error
    }
    pub fn how_to_fix(&self) -> &Vec<String> {
        &self.how_to_fix
    }
    pub fn error_title(&self) -> &str {
        &self.error_title
    }
    pub fn when_error(&self) -> &str {
        &self.when_error
    }
    pub fn error_type(&self) -> &ErrorType {
        &self.error_type
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorType {
    Error,
    Warning,
}

/// Builder struct for ErrorReporter
#[derive(Debug, Clone, Eq, Hash, Serialize, Deserialize, PartialEq)]
pub struct ErrorReporterBuilder<'a> {
    /// The actual error (i.e. A panic message)
    pub actual_error: &'a str,
    /// Why the error happened
    pub why_error: Vec<&'a str>,
    /// How to fix the error
    pub how_to_fix: Vec<&'a str>,
    /// Title of the error
    pub error_title: &'a str,
    /// When the error happened
    pub when_error: &'a str,
    /// Error type
    pub error_type: ErrorType,
}

impl<'a> ErrorReporterBuilder<'a> {
    pub fn build(&self) -> ErrorReporter {
        ErrorReporter {
            actual_error: self.actual_error.to_string(),
            why_error: self.why_error.iter().map(|s| s.to_string()).collect(),
            how_to_fix: self.how_to_fix.iter().map(|s| s.to_string()).collect(),
            error_title: self.error_title.to_string(),
            when_error: format!(
                "The error occurred when {}",
                self.when_error.to_string().to_lowercase()
            ),
            error_type: self.error_type.clone(),
        }
    }
}
