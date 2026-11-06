use eframe::egui;
use crate::scene::Scene;
use crate::ecs::Transform;
use crate::mesh::MeshData;
use crate::material::Material;
use crate::viewport::ViewportState;
use glam::{Vec3, Vec4};

pub struct UIState {
    pub viewport: ViewportState,
    pub show_grid: bool,
    pub wireframe: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            viewport: ViewportState::new(),
            show_grid: true,
            wireframe: false,
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn show_ui(scene: &mut Scene, ui_state: &mut UIState, ctx: &egui::Context) {
    // Left panel: Scene hierarchy
    egui::SidePanel::left("scene_panel").show(ctx, |ui| {
        ui.heading("Scene Hierarchy");
        ui.separator();
        
        if ui.button("🗑 Delete Selected").clicked() {
            scene.remove_selected();
        }
        ui.separator();
        
        for entity in scene.entities() {
            let is_selected = scene.selected_entity == Some(entity);
            let name: String = scene.world.name(entity)
                .map(|n| n.value.clone())
                .unwrap_or_else(|| "Unnamed".to_string());
            
            let label = if is_selected {
                format!("▸ {}", name)
            } else {
                name.clone()
            };
            
            let btn = ui.button(label);
            if btn.clicked() {
                scene.select_entity(entity);
            }
            
            if is_selected {
                ui.add_space(2.0);
            }
        }
        
        ui.separator();
        ui.label(format!("Objects: {}", scene.entities().len()));
    });

    // Right panel: Properties
    egui::SidePanel::right("properties_panel").show(ctx, |ui| {
        ui.heading("Properties");
        ui.separator();
        
        if let Some(entity) = scene.selected_entity {
            if let Some(transform) = scene.world.transform_mut(entity) {
                ui.collapsing("Transform", |ui| {
                    ui.label("Position:");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::DragValue::new(&mut transform.position.x).speed(0.01));
                        ui.label("Y:");
                        ui.add(egui::DragValue::new(&mut transform.position.y).speed(0.01));
                        ui.label("Z:");
                        ui.add(egui::DragValue::new(&mut transform.position.z).speed(0.01));
                    });
                    
                    ui.label("Scale:");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.01).range(0.01..=10.0));
                        ui.label("Y:");
                        ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.01).range(0.01..=10.0));
                        ui.label("Z:");
                        ui.add(egui::DragValue::new(&mut transform.scale.z).speed(0.01).range(0.01..=10.0));
                    });
                });
                
                ui.collapsing("Material", |ui| {
                    if let Some(material) = scene.world.material_mut(entity) {
                        ui.label("Diffuse Color:");
                        ui.color_edit_button_rgb(&mut material.diffuse_color);
                        
                        ui.add(egui::Slider::new(&mut material.metallic, 0.0..=1.0).text("Metallic"));
                        ui.add(egui::Slider::new(&mut material.roughness, 0.0..=1.0).text("Roughness"));
                        
                        ui.label("Emissive:");
                        ui.color_edit_button_rgb(&mut material.emissive);
                    }
                });
                
                // Mesh info
                if let Some(mesh) = scene.world.mesh(entity) {
                    ui.collapsing("Mesh", |ui| {
                        ui.label(format!("Vertices: {}", mesh.vertex_count()));
                        ui.label(format!("Faces: {}", mesh.face_count()));
                        ui.label(format!("Name: {}", mesh.name));
                    });
                }
                
                // Edit mode
                ui.separator();
                if scene.is_edit_mode() {
                    if ui.button("Exit Edit Mode").clicked() {
                        scene.exit_edit_mode();
                    }
                } else {
                    if ui.button("Enter Edit Mode").clicked() {
                        scene.enter_edit_mode();
                    }
                }
            }
        } else {
            ui.label("No object selected");
            ui.label("Select an object from the scene hierarchy or add a new one.");
        }
    });

    // Center: 3D Viewport
    egui::CentralPanel::default().show(ctx, |ui| {
        let response = ui.allocate_ui(ui.available_size(), |ui| {
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
            
            let size = response.rect.size();
            let center = response.rect.center();
            
            // Update camera aspect ratio
            ui_state.viewport.camera.set_aspect(size.x / size.y.max(1.0));
            
            // Background
            painter.rect(response.rect, 0.0, egui::Color32::from_rgb(30, 30, 40), egui::Stroke::NONE);
            
            // Draw grid
            if ui_state.show_grid {
                draw_grid_3d(&painter, center, size, &ui_state.viewport.camera);
            }
            
            // Draw objects with 3D projection
            for entity in scene.entities() {
                let is_selected = scene.selected_entity == Some(entity);
                if let (Some(transform), Some(mesh), Some(material)) = (
                    scene.world.transform(entity),
                    scene.world.mesh(entity),
                    scene.world.material(entity),
                ) {
                    draw_object_3d(
                        &painter,
                        center,
                        size,
                        transform,
                        mesh,
                        material,
                        is_selected,
                        &ui_state.viewport.camera,
                    );
                }
            }
            
            // Draw info overlay
            painter.text(
                response.rect.left_top() + egui::Vec2::new(8.0, 18.0),
                egui::Align2::LEFT_TOP,
                "LMB: Select | RMB: Orbit | Scroll: Zoom",
                egui::FontId::monospace(11.0),
                egui::Color32::from_rgb(180, 180, 200),
            );
            
            response
        });
        
        // Handle viewport interaction
        if response.inner.hovered {
            let mut orbit_delta = egui::Vec2::ZERO;
            ctx.input(|i| {
                if i.pointer.secondary_down() {
                    if let Some(pos) = i.pointer.interact_pos() {
                        let last = ui_state.viewport.last_mouse.unwrap_or(pos);
                        orbit_delta = pos - last;
                        ui_state.viewport.last_mouse = Some(pos);
                    }
                } else {
                    ui_state.viewport.last_mouse = None;
                }
            });
            if orbit_delta != egui::Vec2::ZERO {
                ui_state.viewport.camera.orbit(orbit_delta.x, orbit_delta.y);
            }
        }
        
        // Handle scroll zoom
        let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
        if response.inner.hovered && scroll.abs() > 0.01 {
            ui_state.viewport.camera.zoom(scroll * 0.01);
        }
    });
}

fn project_3d(
    point: Vec3,
    camera: &crate::viewport::Camera,
    screen_center: egui::Pos2,
    screen_size: egui::Vec2,
) -> Option<egui::Pos2> {
    let vp = camera.view_projection();
    let clip = vp * Vec4::new(point.x, point.y, point.z, 1.0);
    
    // Perspective divide
    if clip.w <= 0.0 {
        return None;
    }
    
    let ndc_x = clip.x / clip.w;
    let ndc_y = clip.y / clip.w;
    
    // Convert to screen space
    let screen_x = screen_center.x + ndc_x * screen_size.x * 0.5;
    let screen_y = screen_center.y - ndc_y * screen_size.y * 0.5; // Flip Y
    
    Some(egui::Pos2::new(screen_x, screen_y))
}

fn draw_grid_3d(
    painter: &egui::Painter,
    center: egui::Pos2,
    size: egui::Vec2,
    camera: &crate::viewport::Camera,
) {
    let grid_size = 10;
    let half = grid_size as f32 / 2.0;
    
    for i in 0..=grid_size {
        let pos = -half + i as f32;
        
        // Lines along X
        let p1 = Vec3::new(-half, 0.0, pos);
        let p2 = Vec3::new(half, 0.0, pos);
        if let (Some(s1), Some(s2)) = (project_3d(p1, camera, center, size), project_3d(p2, camera, center, size)) {
            painter.line_segment(
                [s1, s2],
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(60, 60, 80)),
            );
        }
        
        // Lines along Z
        let p1 = Vec3::new(pos, 0.0, -half);
        let p2 = Vec3::new(pos, 0.0, half);
        if let (Some(s1), Some(s2)) = (project_3d(p1, camera, center, size), project_3d(p2, camera, center, size)) {
            painter.line_segment(
                [s1, s2],
                egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(60, 60, 80)),
            );
        }
    }
}

fn draw_object_3d(
    painter: &egui::Painter,
    center: egui::Pos2,
    size: egui::Vec2,
    transform: &Transform,
    mesh: &MeshData,
    material: &Material,
    is_selected: bool,
    camera: &crate::viewport::Camera,
) {
    let world_matrix = transform.to_matrix();
    let color = material.to_egui_color();
    
    let final_color = if is_selected {
        egui::Color32::from_rgb(
            (color.r() as f32 * 1.3).min(255.0) as u8,
            (color.g() as f32 * 1.3).min(255.0) as u8,
            (color.b() as f32 * 1.3).min(255.0) as u8,
        )
    } else {
        color
    };
    
    // Project all vertices
    let mut screen_points: Vec<egui::Pos2> = Vec::new();
    for vertex in &mesh.vertices {
        let world_pos = world_matrix * Vec4::new(
            vertex.position[0], vertex.position[1], vertex.position[2], 1.0
        );
        let world_pos = Vec3::new(world_pos.x, world_pos.y, world_pos.z);
        
        if let Some(screen_pos) = project_3d(world_pos, camera, center, size) {
            screen_points.push(screen_pos);
        }
    }
    
    if screen_points.is_empty() {
        return;
    }
    
    // Draw filled triangles first
    if !mesh.indices.is_empty() && !is_selected {
        for i in (0..mesh.indices.len()).step_by(3) {
            if i + 2 < mesh.indices.len() {
                let a = mesh.indices[i] as usize;
                let b = mesh.indices[i + 1] as usize;
                let c = mesh.indices[i + 2] as usize;
                
                if a < screen_points.len() && b < screen_points.len() && c < screen_points.len() {
                    // Draw triangle using line segments for filled effect
                    let stroke = egui::Stroke::new(1.0_f32, final_color.linear_multiply(0.7));
                    painter.line_segment([screen_points[a], screen_points[b]], stroke);
                    painter.line_segment([screen_points[b], screen_points[c]], stroke);
                    painter.line_segment([screen_points[c], screen_points[a]], stroke);
                }
            }
        }
    }
    
    // Draw wireframe edges
    if !mesh.indices.is_empty() {
        for i in (0..mesh.indices.len()).step_by(3) {
            if i + 2 < mesh.indices.len() {
                let a = mesh.indices[i] as usize;
                let b = mesh.indices[i + 1] as usize;
                let c = mesh.indices[i + 2] as usize;
                
                if a < screen_points.len() && b < screen_points.len() && c < screen_points.len() {
                    let stroke = egui::Stroke::new(
                        if is_selected { 1.5_f32 } else { 0.5_f32 },
                        if is_selected { egui::Color32::WHITE } else { final_color.linear_multiply(0.5) },
                    );
                    
                    painter.line_segment([screen_points[a], screen_points[b]], stroke);
                    painter.line_segment([screen_points[b], screen_points[c]], stroke);
                    painter.line_segment([screen_points[c], screen_points[a]], stroke);
                }
            }
        }
    }
    
    // Draw label at center
    let mut center_pos = Vec3::ZERO;
    for vertex in &mesh.vertices {
        center_pos += Vec3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
    }
    if !mesh.vertices.is_empty() {
        center_pos /= mesh.vertices.len() as f32;
        let world_center = world_matrix * Vec4::new(center_pos.x, center_pos.y, center_pos.z, 0.0);
        if let Some(screen_center) = project_3d(
            Vec3::new(world_center.x, world_center.y, world_center.z),
            camera, center, size
        ) {
            painter.text(
                screen_center + egui::Vec2::new(0.0, -15.0),
                egui::Align2::CENTER_CENTER,
                mesh.name.clone(),
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );
        }
    }
}
