//! Error types for gba-core

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Prompt error: {0}")]
    PromptError(#[from] gba_pm::PromptError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
