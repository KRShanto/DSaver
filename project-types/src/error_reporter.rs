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

