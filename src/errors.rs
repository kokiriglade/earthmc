//! # Errors
//!
//! Errors that can occur when interacting with the EarthMC client.
use thiserror::Error;

/// Errors that can occur when interacting with the EarthMC client.
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),
    #[error("JSON deserialization failed: {source}\nSnippet:\n{snippet}")]
    DeserializationWithSnippet {
        source: serde_json::Error,
        snippet: String,
    },
    #[error("Too many retries")]
    TooManyRetry(Vec<Error>),
}

/// Given a piece of text and a line/column, return a small snippet
pub fn snippet_around(
    full: &str,
    line: usize,
    column: usize,
    context: usize,
) -> String {
    let mut current_line = 1;
    let mut idx = 0;
    let bytes = full.as_bytes();

    while current_line < line && idx < bytes.len() {
        if bytes[idx] == b'\n' {
            current_line += 1;
        }
        idx += 1;
    }

    let mut error_byte = idx.saturating_add(column.saturating_sub(1));
    if error_byte > bytes.len() {
        error_byte = bytes.len();
    }

    let start = error_byte.saturating_sub(context);
    let end = (error_byte + context).min(full.len());

    let mut result = String::new();
    if start > 0 {
        result.push_str("…");
    }
    result.push_str(&full[start..end]);
    if end < full.len() {
        result.push_str("…");
    }
    result
}
