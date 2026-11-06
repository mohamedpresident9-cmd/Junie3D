use std::fs;
use crate::scene::Scene;
use crate::mesh::MeshData;
use crate::ecs::{Entity, Transform, Name};
use crate::material::Material;

/// Import a mesh from an OBJ file
pub fn import_obj(path: &str) -> Result<MeshData, String> {
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions::default()
    )
    .map_err(|e| format!("Failed to load OBJ: {}", e))?;
    
    if models.is_empty() {
        return Err("No models found in OBJ file".to_string());
    }
    
    let model = &models[0];
    let mesh = &model.mesh;
    
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // tobj stores positions/normals as flat Vec<f64>: [x,y,z, x,y,z, ...]
    let pos_count = mesh.positions.len() / 3;
    
    // Build vertex and index arrays using rounded keys to avoid f64 hashing issues
    let mut vertex_map: std::collections::HashMap<[i64; 6], u32> = std::collections::HashMap::new();
    
    for i in 0..pos_count {
        let px = mesh.positions[i * 3] as f32;
        let py = mesh.positions[i * 3 + 1] as f32;
        let pz = mesh.positions[i * 3 + 2] as f32;
        
        let (nx, ny, nz) = if mesh.normals.len() >= 3 {
            let ni = if i < mesh.normals.len() / 3 { i } else { 0 };
            (
                mesh.normals[ni * 3] as f32,
                mesh.normals[ni * 3 + 1] as f32,
                mesh.normals[ni * 3 + 2] as f32,
            )
        } else {
            (0.0, 0.0, 1.0)
        };
        
        // Round to 6 decimal places and convert to i64 for hashing
        let key = [
            (px * 1_000_000.0).round() as i64,
            (py * 1_000_000.0).round() as i64,
            (pz * 1_000_000.0).round() as i64,
            (nx * 1_000_000.0).round() as i64,
            (ny * 1_000_000.0).round() as i64,
            (nz * 1_000_000.0).round() as i64,
        ];
        
        if let Some(&idx) = vertex_map.get(&key) {
            indices.push(idx);
        } else {
            let idx = vertices.len() as u32;
            vertex_map.insert(key, idx);
            vertices.push(crate::mesh::Vertex {
                position: [px, py, pz],
                normal: [nx, ny, nz],
            });
            indices.push(idx);
        }
    }
    
    // If the mesh has indices, use them; otherwise triangulate from vertex order
    if !mesh.indices.is_empty() {
        // Use the mesh indices directly (they reference position indices)
        // We already built our own indices above, so use mesh.indices if available
        indices.clear();
        for &idx in &mesh.indices {
            // Map tobj index to our vertex map
            let i = idx as usize;
            if i < pos_count {
                let px = mesh.positions[i * 3] as f32;
                let py = mesh.positions[i * 3 + 1] as f32;
                let pz = mesh.positions[i * 3 + 2] as f32;
                
                let (nx, ny, nz) = if mesh.normals.len() >= 3 {
                    let ni = if i < mesh.normals.len() / 3 { i } else { 0 };
                    (
                        mesh.normals[ni * 3] as f32,
                        mesh.normals[ni * 3 + 1] as f32,
                        mesh.normals[ni * 3 + 2] as f32,
                    )
                } else {
                    (0.0, 0.0, 1.0)
                };
                
                let key = [
                    (px * 1_000_000.0).round() as i64,
                    (py * 1_000_000.0).round() as i64,
                    (pz * 1_000_000.0).round() as i64,
                    (nx * 1_000_000.0).round() as i64,
                    (ny * 1_000_000.0).round() as i64,
                    (nz * 1_000_000.0).round() as i64,
                ];
                
                if let Some(&our_idx) = vertex_map.get(&key) {
                    indices.push(our_idx);
                }
            }
        }
    }
    
    Ok(MeshData {
        vertices,
        indices,
        name: std::path::Path::new(path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Imported".to_string()),
    })
}

/// Add an imported OBJ to the scene
pub fn import_obj_to_scene(scene: &mut Scene, path: &str) -> Result<Entity, String> {
    let mesh = import_obj(path)?;
    let name = mesh.name.clone();
    let entity = scene.world.create_entity(&name);
    scene.world.set_mesh(entity, mesh);
    scene.select_entity(entity);
    Ok(entity)
}

/// Save the scene to a .junie3d binary file
pub fn save_project(scene: &Scene, path: &str) -> Result<(), String> {
    let data = bincode::serialize(&SceneData {
        entities: scene.entities(),
        transforms: scene
            .entities()
            .iter()
            .filter_map(|e| scene.world.transform(*e).map(|t| (*e, t.clone())))
            .collect(),
        names: scene
            .entities()
            .iter()
            .filter_map(|e| scene.world.name(*e).map(|n| (*e, n.clone())))
            .collect(),
        materials: scene
            .entities()
            .iter()
            .filter_map(|e| scene.world.material(*e).map(|m| (*e, m.clone())))
            .collect(),
    })
    .map_err(|e| format!("Serialization failed: {}", e))?;
    
    fs::write(path, data)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

/// Load a scene from a .junie3d binary file
pub fn load_project(path: &str) -> Result<Scene, String> {
    let data = fs::read(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let scene_data: SceneData = bincode::deserialize(&data)
        .map_err(|e| format!("Deserialization failed: {}", e))?;
    
    let mut scene = Scene::new();
    
    for (entity, transform) in &scene_data.transforms {
        let name = scene_data
            .names
            .iter()
            .find(|(e, _)| e == entity)
            .map(|(_, n)| n.value.as_str())
            .unwrap_or("Loaded");
        
        let e = scene.world.create_entity(name);
        scene.world.transform_mut(e).map(|t| *t = transform.clone());
        
        if let Some((_, material)) = scene_data.materials.iter().find(|(e, _)| e == entity) {
            scene.world.set_material(e, material.clone());
        }
    }
    
    Ok(scene)
}

/// Export the scene to OBJ format
pub fn export_obj(scene: &Scene, path: &str) -> Result<(), String> {
    let mut obj = String::new();
    obj.push_str("# Junie 3D Export\n");
    
    for entity in scene.entities() {
        if let Some(mesh) = scene.world.mesh(entity) {
            if let Some(name) = scene.world.name(entity) {
                obj.push_str(&format!("\n# Object: {}\n", name.value));
            }
            
            for v in &mesh.vertices {
                obj.push_str(&format!(
                    "v {} {} {}\n",
                    v.position[0], v.position[1], v.position[2]
                ));
            }
            
            for v in &mesh.vertices {
                obj.push_str(&format!(
                    "vn {} {} {}\n",
                    v.normal[0], v.normal[1], v.normal[2]
                ));
            }
            
            for i in (0..mesh.indices.len()).step_by(3) {
                let a = mesh.indices[i] as usize + 1;
                let b = mesh.indices[i + 1] as usize + 1;
                let c = mesh.indices[i + 2] as usize + 1;
                obj.push_str(&format!("f {}/{} {}/{} {}/{}\n", a, a, b, b, c, c));
            }
        }
    }
    
    fs::write(path, obj)
        .map_err(|e| format!("Failed to write OBJ: {}", e))?;
    
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SceneData {
    entities: Vec<Entity>,
    transforms: Vec<(Entity, Transform)>,
    names: Vec<(Entity, Name)>,
    materials: Vec<(Entity, Material)>,
}
