//! Graph operator jobs exposed by the application layer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphJob {
    Impact { id: String },
    Tree { status: String, compact: bool },
}

impl GraphJob {
    pub fn command_group(&self) -> &'static str {
        "graph"
    }
}
