//! # gba-pm
//!
//! Prompt manager for GBA (Geektime Bootcamp Agent).
//!
//! This crate provides template-based prompt management using minijinja.

mod error;
mod kind;
mod manager;
mod template;

pub use error::*;
pub use kind::*;
pub use manager::*;
pub use template::*;
