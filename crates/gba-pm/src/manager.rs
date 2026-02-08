//! Prompt manager implementation

use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct PromptManager {
    #[builder(default)]
    template_dir: Option<String>,
}

impl PromptManager {
    // TODO: Implement prompt management methods
}
