//! Agent configuration

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AgentConfig {
    #[builder(default = "claude-sonnet-4-20250514".to_string())]
    pub model: String,

    #[builder(default)]
    pub api_key: Option<String>,

    #[builder(default = 4096)]
    pub max_tokens: u32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self::builder().build()
    }
}
