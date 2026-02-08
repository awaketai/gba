//! Application state

use gba_core::AgentConfig;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct App {
    #[builder(default)]
    config: AgentConfig,
}

impl App {
    // TODO: Implement app methods
}
