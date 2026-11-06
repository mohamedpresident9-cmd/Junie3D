mod ecs;
mod scene;
mod mesh;
mod material;
mod viewport;
mod ui;
mod file_io;

use eframe::egui;
use scene::Scene;
use ui::UIState;

struct Junie3DApp {
    scene: Scene,
    ui_state: UIState,
}

impl Junie3DApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        env_logger::init();
        Self {
            scene: Scene::new(),
            ui_state: UIState::new(),
        }
    }
}

impl eframe::App for Junie3DApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        self.scene = Scene::new();
                    }
                    if ui.button("Import OBJ...").clicked() {
                        // TODO: file dialog
                    }
                    if ui.button("Save Project").clicked() {
                        // TODO: save project
                    }
                    if ui.button("Export OBJ").clicked() {
                        // TODO: export
                    }
                });
                ui.menu_button("Add", |ui| {
                    if ui.button("Cube").clicked() {
                        self.scene.add_cube();
                    }
                    if ui.button("Sphere").clicked() {
                        self.scene.add_sphere();
                    }
                    if ui.button("Cylinder").clicked() {
                        self.scene.add_cylinder();
                    }
                });
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.ui_state.show_grid, "Show Grid");
                    ui.checkbox(&mut self.ui_state.wireframe, "Wireframe");
                    if ui.button("Reset Camera").clicked() {
                        self.ui_state.viewport.camera.reset();
                    }
                });
                ui.add_space(16.0);
                ui.label("Junie 3D Editor");
            });
        });

        ui::show_ui(&mut self.scene, &mut self.ui_state, ctx);

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Junie 3D Editor"),
        ..Default::default()
    };
    eframe::run_native(
        "Junie 3D Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(Junie3DApp::new(cc)))),
    )
}
