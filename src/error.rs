//! Application error taxonomy for fetch/parse boundaries and `ErrorBoundary` integration.
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Recoverable errors from JSON fetch/parse and in-app logic checks; serializable for `create_resource`.
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum AppError {
    #[error("fetch failed: {0}")]
    Fetch(String),
    #[error("parse failed: {0}")]
    Parse(String),
    #[error("logic error: {0}")]
    Logic(String),
}

impl AppError {
    /// Network or transport failure loading a static JSON asset.
    pub fn fetch(msg: impl Into<String>) -> Self {
        Self::Fetch(msg.into())
    }

    /// JSON did not deserialize into the expected type.
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }

    /// Invalid parameters or missing data before a fetch runs.
    pub fn logic(msg: impl Into<String>) -> Self {
        Self::Logic(msg.into())
    }
}
