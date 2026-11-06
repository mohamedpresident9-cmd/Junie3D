use std::collections::HashMap;
use crate::ecs::{Entity, Transform, Name, AppEvent, EventBus};
use crate::mesh::MeshData;
use crate::material::Material;

/// The ECS world holds all components and processes events
pub struct World {
    pub transforms: HashMap<Entity, Transform>,
    pub names: HashMap<Entity, Name>,
    pub meshes: HashMap<Entity, MeshData>,
    pub materials: HashMap<Entity, Material>,
    pub event_bus: EventBus,
    next_entity_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            transforms: HashMap::new(),
            names: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
            event_bus: EventBus::new(),
            next_entity_id: 0,
        }
    }

    /// Create a new entity with default components
    pub fn create_entity(&mut self, name: &str) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        
        self.transforms.insert(entity, Transform::new());
        self.names.insert(entity, Name::new(name));
        self.materials.insert(entity, Material::default());
        
        self.event_bus.publish(AppEvent::AddEntity(entity));
        entity
    }

    /// Remove an entity and all its components
    pub fn remove_entity(&mut self, entity: Entity) {
        self.transforms.remove(&entity);
        self.names.remove(&entity);
        self.meshes.remove(&entity);
        self.materials.remove(&entity);
        self.event_bus.publish(AppEvent::RemoveEntity(entity));
    }

    /// Get a mutable transform for an entity
    pub fn transform_mut(&mut self, entity: Entity) -> Option<&mut Transform> {
        self.transforms.get_mut(&entity)
    }

    /// Get a transform for an entity
    pub fn transform(&self, entity: Entity) -> Option<&Transform> {
        self.transforms.get(&entity)
    }

    /// Get a name for an entity
    pub fn name(&self, entity: Entity) -> Option<&Name> {
        self.names.get(&entity)
    }

    /// Set the mesh for an entity
    pub fn set_mesh(&mut self, entity: Entity, mesh: MeshData) {
        self.meshes.insert(entity, mesh);
    }

    /// Get a mesh for an entity
    pub fn mesh(&self, entity: Entity) -> Option<&MeshData> {
        self.meshes.get(&entity)
    }

    /// Get a mutable mesh for an entity
    pub fn mesh_mut(&mut self, entity: Entity) -> Option<&mut MeshData> {
        self.meshes.get_mut(&entity)
    }

    /// Set the material for an entity
    pub fn set_material(&mut self, entity: Entity, material: Material) {
        self.materials.insert(entity, material);
    }

    /// Get a material for an entity
    pub fn material(&self, entity: Entity) -> Option<&Material> {
        self.materials.get(&entity)
    }

    /// Get a mutable material for an entity
    pub fn material_mut(&mut self, entity: Entity) -> Option<&mut Material> {
        self.materials.get_mut(&entity)
    }

    /// Get all entity IDs
    pub fn entity_ids(&self) -> Vec<Entity> {
        self.transforms.keys().cloned().collect()
    }

    /// Process pending events
    pub fn process_events(&mut self) {
        let events = self.event_bus.drain_events();
        for event in events {
            match event {
                AppEvent::TransformPosition(entity, pos) => {
                    if let Some(t) = self.transforms.get_mut(&entity) {
                        t.position = pos;
                    }
                }
                AppEvent::TransformRotation(entity, euler) => {
                    if let Some(t) = self.transforms.get_mut(&entity) {
                        t.rotation = glam::Quat::from_euler(
                            glam::EulerRot::XYZ,
                            euler.x, euler.y, euler.z
                        );
                    }
                }
                AppEvent::TransformScale(entity, scale) => {
                    if let Some(t) = self.transforms.get_mut(&entity) {
                        t.scale = scale;
                    }
                }
                _ => {} // Other events handled by scene
            }
        }
    }

    /// Check if an entity exists
    pub fn has_entity(&self, entity: Entity) -> bool {
        self.transforms.contains_key(&entity)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
