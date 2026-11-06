use serde::{Serialize, Deserialize};

/// A vertex with position and normal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

/// Mesh data: vertices and indices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub name: String,
}

impl MeshData {
    pub fn new(name: &str) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.indices.len() / 3
    }

    /// Generate a unit cube mesh
    pub fn cube() -> Self {
        // Front face
        let f0 = Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0] };
        let f1 = Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0, 0.0, 1.0] };
        let f2 = Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0] };
        let f3 = Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 0.0, 1.0] };
        // Back face
        let b0 = Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0,-1.0] };
        let b1 = Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0,-1.0] };
        let b2 = Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 0.0,-1.0] };
        let b3 = Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0,-1.0] };
        // Top face
        let t0 = Vertex { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0] };
        let t1 = Vertex { position: [ 0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0] };
        let t2 = Vertex { position: [ 0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0] };
        let t3 = Vertex { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0] };
        // Bottom face
        let bt0 = Vertex { position: [-0.5, -0.5, -0.5], normal: [0.0,-1.0, 0.0] };
        let bt1 = Vertex { position: [ 0.5, -0.5, -0.5], normal: [0.0,-1.0, 0.0] };
        let bt2 = Vertex { position: [ 0.5, -0.5,  0.5], normal: [0.0,-1.0, 0.0] };
        let bt3 = Vertex { position: [-0.5, -0.5,  0.5], normal: [0.0,-1.0, 0.0] };
        // Right face
        let r0 = Vertex { position: [ 0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0] };
        let r1 = Vertex { position: [ 0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0] };
        let r2 = Vertex { position: [ 0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0] };
        let r3 = Vertex { position: [ 0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0] };
        // Left face
        let l0 = Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0] };
        let l1 = Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0, 0.0, 0.0] };
        let l2 = Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0] };
        let l3 = Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0, 0.0, 0.0] };

        let vertices = vec![
            f0, f1, f2, f3,
            b0, b1, b2, b3,
            t0, t1, t2, t3,
            bt0, bt1, bt2, bt3,
            r0, r1, r2, r3,
            l0, l1, l2, l3,
        ];

        let indices: Vec<u32> = vec![
            // Front
            0, 1, 2,  0, 2, 3,
            // Back
            4, 5, 6,  4, 6, 7,
            // Top
            8, 9, 10,  8, 10, 11,
            // Bottom
            12, 13, 14,  12, 14, 15,
            // Right
            16, 17, 18,  16, 18, 19,
            // Left
            20, 21, 22,  20, 22, 23,
        ];

        Self { vertices, indices, name: "Cube".to_string() }
    }

    /// Generate a sphere mesh with the given segments
    pub fn sphere(segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let segments = segments.max(3) as f32;
        let rings = (segments * 0.5).floor() as f32;

        for ring in 0..=(rings as u32) {
            let v = ring as f32 / rings;
            let theta = v * std::f32::consts::PI;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for seg in 0..=segments as u32 {
                let u = seg as f32 / segments;
                let phi = u * 2.0 * std::f32::consts::PI;
                let x = sin_theta * phi.cos();
                let y = cos_theta;
                let z = sin_theta * phi.sin();

                vertices.push(Vertex {
                    position: [x, y, z],
                    normal: [x, y, z],
                });
            }
        }

        for ring in 0..rings as u32 {
            let ring = ring as u32;
            let next = ring + 1;
            let base = ring * (segments as u32 + 1);
            let next_base = next * (segments as u32 + 1);

            for seg in 0..segments as u32 {
                let a = base + seg;
                let b = base + seg + 1;
                let c = next_base + seg;
                let d = next_base + seg + 1;

                indices.push(a);
                indices.push(c);
                indices.push(b);
                indices.push(b);
                indices.push(c);
                indices.push(d);
            }
        }

        Self { vertices, indices, name: "Sphere".to_string() }
    }

    /// Generate a cylinder mesh
    pub fn cylinder(segments: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let segments = segments.max(3) as f32;

        // Side vertices
        for seg in 0..=segments as u32 {
            let u = seg as f32 / segments;
            let phi = u * 2.0 * std::f32::consts::PI;
            let x = phi.cos();
            let z = phi.sin();

            // Top vertex
            vertices.push(Vertex {
                position: [x, 0.5, z],
                normal: [x, 0.0, z],
            });
            // Bottom vertex
            vertices.push(Vertex {
                position: [x, -0.5, z],
                normal: [x, 0.0, z],
            });
        }

        // Side indices
        for seg in 0..segments as u32 {
            let a = seg * 2;
            let b = seg * 2 + 1;
            let c = (seg + 1) * 2;
            let d = (seg + 1) * 2 + 1;

            indices.push(a);
            indices.push(c);
            indices.push(b);
            indices.push(b);
            indices.push(c);
            indices.push(d);
        }

        // Top cap
        let top_center = (segments as u32 * 2) as u32;
        vertices.push(Vertex {
            position: [0.0, 0.5, 0.0],
            normal: [0.0, 1.0, 0.0],
        });
        for seg in 0..segments as u32 {
            indices.push(top_center);
            indices.push(seg * 2 + 1);
            indices.push(seg * 2);
        }

        // Bottom cap
        let bottom_center = (segments as u32 * 2 + 1) as u32;
        vertices.push(Vertex {
            position: [0.0, -0.5, 0.0],
            normal: [0.0, -1.0, 0.0],
        });
        for seg in 0..segments as u32 {
            indices.push(bottom_center);
            indices.push(seg * 2 + 1);
            indices.push(seg * 2 + 3);
        }

        Self { vertices, indices, name: "Cylinder".to_string() }
    }

    /// Generate a grid mesh for the viewport
    pub fn grid(size: f32, divisions: u32) -> Self {
        let mut vertices = Vec::new();
        let indices: Vec<u32> = Vec::new();
        let half = size / 2.0;
        let step = size / divisions as f32;

        // Create a line grid
        for i in 0..=divisions {
            let pos = -half + i as f32 * step;
            // Lines along X
            vertices.push(Vertex {
                position: [-half, 0.0, pos],
                normal: [0.0, 1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [half, 0.0, pos],
                normal: [0.0, 1.0, 0.0],
            });
            // Lines along Z
            vertices.push(Vertex {
                position: [pos, 0.0, -half],
                normal: [0.0, 1.0, 0.0],
            });
            vertices.push(Vertex {
                position: [pos, 0.0, half],
                normal: [0.0, 1.0, 0.0],
            });
        }

        Self { vertices, indices, name: "Grid".to_string() }
    }
}
