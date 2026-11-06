use glam::{Vec3, Mat4, Vec4Swizzles};

/// Camera for the 3D viewport with orbit, pan, and zoom
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    // Orbit parameters
    pub orbit_angle_x: f32,
    pub orbit_angle_y: f32,
    pub orbit_distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 2.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 45.0_f32.to_radians(),
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
            orbit_angle_x: 0.5,
            orbit_angle_y: 0.3,
            orbit_distance: 8.0,
        }
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect.max(0.01);
    }

    /// Orbit the camera around the target
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.orbit_angle_x += dx * 0.01;
        self.orbit_angle_y += dy * 0.01;
        // Clamp vertical angle to avoid flipping
        self.orbit_angle_y = self.orbit_angle_y.clamp(-1.5, 1.5);
        self.update_position();
    }

    /// Pan the camera target
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let view = self.view_matrix();
        let right = (view * glam::Vec4::new(1.0, 0.0, 0.0, 0.0)).xyz().normalize();
        let up = (view * glam::Vec4::new(0.0, 1.0, 0.0, 0.0)).xyz().normalize();
        let pan_speed = self.orbit_distance * 0.002;
        self.target += right * (-dx * pan_speed) + up * (dy * pan_speed);
        self.update_position();
    }

    /// Zoom the camera
    pub fn zoom(&mut self, delta: f32) {
        self.orbit_distance *= 1.0 + delta * 0.1;
        self.orbit_distance = self.orbit_distance.clamp(0.5, 100.0);
        self.update_position();
    }

    /// Update camera position based on orbit parameters
    fn update_position(&mut self) {
        let x = self.orbit_distance * self.orbit_angle_y.cos() * self.orbit_angle_x.sin();
        let y = self.orbit_distance * self.orbit_angle_y.sin();
        let z = self.orbit_distance * self.orbit_angle_y.cos() * self.orbit_angle_x.cos();
        self.position = self.target + Vec3::new(x, y, z);
    }

    /// Compute the view matrix
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_to_rh(self.position, self.target, self.up)
    }

    /// Compute the projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Compute the view-projection matrix
    pub fn view_projection(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Reset camera to default position
    pub fn reset(&mut self) {
        self.orbit_angle_x = 0.5;
        self.orbit_angle_y = 0.3;
        self.orbit_distance = 8.0;
        self.target = Vec3::ZERO;
        self.update_position();
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

/// Viewport state for interaction
pub struct ViewportState {
    pub camera: Camera,
    pub is_orbiting: bool,
    pub is_panning: bool,
    pub last_mouse: Option<egui::Pos2>,
}

impl ViewportState {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            is_orbiting: false,
            is_panning: false,
            last_mouse: None,
        }
    }
}

impl Default for ViewportState {
    fn default() -> Self {
        Self::new()
    }
}
