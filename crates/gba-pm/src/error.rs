//! Error types for gba-pm

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PromptError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Template render error: {0}")]
    RenderError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
