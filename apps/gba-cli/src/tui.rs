//! TUI (Terminal User Interface) implementation

#![allow(unused_imports)]

use anyhow::Result;
use ratatui::prelude::*;

pub struct Tui {
    // TODO: Add TUI state
}

impl Tui {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&mut self) -> Result<()> {
        // TODO: Implement TUI loop
        Ok(())
    }
}

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}
