use crate::ecs::{Entity, World};
use crate::mesh::MeshData;

/// Scene manages all objects and selection state
pub struct Scene {
    pub world: World,
    pub selected_entity: Option<Entity>,
    pub edit_mode_entity: Option<Entity>,
    object_counter: std::collections::HashMap<String, u32>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            selected_entity: None,
            edit_mode_entity: None,
            object_counter: std::collections::HashMap::new(),
        }
    }

    fn next_name(&mut self, base: &str) -> String {
        let counter = self.object_counter.entry(base.to_string()).or_insert(0);
        let name = format!("{} {}", base, counter);
        *counter += 1;
        name
    }

    /// Add a cube to the scene
    pub fn add_cube(&mut self) {
        let name = self.next_name("Cube");
        let entity = self.world.create_entity(&name);
        self.world.set_mesh(entity, MeshData::cube());
        self.select_entity(entity);
    }

    /// Add a sphere to the scene
    pub fn add_sphere(&mut self) {
        let name = self.next_name("Sphere");
        let entity = self.world.create_entity(&name);
        self.world.set_mesh(entity, MeshData::sphere(32));
        self.select_entity(entity);
    }

    /// Add a cylinder to the scene
    pub fn add_cylinder(&mut self) {
        let name = self.next_name("Cylinder");
        let entity = self.world.create_entity(&name);
        self.world.set_mesh(entity, MeshData::cylinder(32));
        self.select_entity(entity);
    }

    /// Select an entity
    pub fn select_entity(&mut self, entity: Entity) {
        self.selected_entity = Some(entity);
    }

    /// Deselect all
    pub fn deselect_all(&mut self) {
        self.selected_entity = None;
        self.edit_mode_entity = None;
    }

    /// Remove the selected entity
    pub fn remove_selected(&mut self) {
        if let Some(entity) = self.selected_entity {
            self.world.remove_entity(entity);
            self.deselect_all();
        }
    }

    /// Enter edit mode for the selected entity
    pub fn enter_edit_mode(&mut self) {
        if let Some(entity) = self.selected_entity {
            self.edit_mode_entity = Some(entity);
        }
    }

    /// Exit edit mode
    pub fn exit_edit_mode(&mut self) {
        self.edit_mode_entity = None;
    }

    /// Get all entities in the scene
    pub fn entities(&self) -> Vec<Entity> {
        self.world.entity_ids()
    }

    /// Check if in edit mode
    pub fn is_edit_mode(&self) -> bool {
        self.edit_mode_entity.is_some()
    }
}
