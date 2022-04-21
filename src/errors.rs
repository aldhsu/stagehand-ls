use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum SHError {
    PathNotFound(String),
    NoAlternate(String),
    SerdeError(String),
}

impl From<serde_json::Error> for SHError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e.to_string())
    }
}
