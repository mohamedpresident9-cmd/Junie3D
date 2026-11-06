use serde::{Serialize, Deserialize};

/// Unique identifier for an entity in the scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entity {
    pub id: u64,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
