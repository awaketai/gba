//! # gba-pm
//!
//! Prompt manager for GBA (Geektime Bootcamp Agent).
//!
//! This crate provides template-based prompt management using minijinja.

mod error;
mod manager;
mod template;

pub use error::*;
pub use manager::*;
pub use template::*;
