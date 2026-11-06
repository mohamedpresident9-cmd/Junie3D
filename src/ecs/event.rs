use serde::{Serialize, Deserialize};
use crate::ecs::Entity;
use glam::Vec3;

/// Events that drive the ECS architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    /// Add a new entity to the scene
    AddEntity(Entity),
    /// Remove an entity from the scene
    RemoveEntity(Entity),
    /// Select an entity
    SelectEntity(Entity),
    /// Deselect all entities
    DeselectAll,
    /// Transform an entity's position
    TransformPosition(Entity, Vec3),
    /// Transform an entity's rotation (Euler angles in radians)
    TransformRotation(Entity, Vec3),
    /// Transform an entity's scale
    TransformScale(Entity, Vec3),
    /// Change an entity's material
    ChangeMaterial(Entity, u32),
    /// Enter edit mode for an entity
    EnterEditMode(Entity),
    /// Exit edit mode
    ExitEditMode,
    /// Move vertices of a mesh
    MoveVertices(Entity, Vec<usize>, Vec3),
    /// Extrude faces
    ExtrudeFaces(Entity, Vec<usize>, Vec3),
    /// Delete selected elements
    DeleteElements(Entity),
    /// Import a file
    ImportFile(String),
    /// Save project
    SaveProject,
    /// Load project
    LoadProject(String),
}

/// Event bus for publishing and subscribing to events
pub struct EventBus {
    pending_events: Vec<AppEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            pending_events: Vec::new(),
        }
    }

    pub fn publish(&mut self, event: AppEvent) {
        self.pending_events.push(event);
    }

    pub fn drain_events(&mut self) -> Vec<AppEvent> {
        self.pending_events.drain(..).collect()
    }

    pub fn has_pending(&self) -> bool {
        !self.pending_events.is_empty()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
