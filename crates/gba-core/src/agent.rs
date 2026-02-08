//! Agent implementation

use typed_builder::TypedBuilder;

use crate::AgentConfig;

#[derive(Debug, TypedBuilder)]
pub struct Agent {
    config: AgentConfig,

    #[builder(default)]
    name: Option<String>,
}

impl Agent {
    // TODO: Implement agent methods
}
