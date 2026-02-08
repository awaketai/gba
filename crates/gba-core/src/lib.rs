//! # gba-core
//!
//! Core execution engine for GBA (Geektime Bootcamp Agent).
//!
//! This crate provides the core agent execution logic using claude-agent-sdk-rs.

mod agent;
mod config;
mod error;
mod executor;

pub use agent::*;
pub use config::*;
pub use error::*;
pub use executor::*;
