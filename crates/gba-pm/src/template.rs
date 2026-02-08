//! Template utilities

use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct Template {
    name: String,
    content: String,
}

impl Template {
    // TODO: Implement template methods
}
