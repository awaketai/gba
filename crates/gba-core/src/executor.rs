//! Agent executor

use typed_builder::TypedBuilder;

use crate::Agent;

#[derive(Debug, TypedBuilder)]
pub struct Executor {
    agent: Agent,
}

impl Executor {
    // TODO: Implement executor methods
}
