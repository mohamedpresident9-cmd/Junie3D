# Junie 3D Editor

A Blender-like 3D creation application built in Rust with **egui** for the UI and **glam** for math.

## Features

- **3D Viewport** with orbit, pan, and zoom camera navigation
- **Mesh Editing** — vertex, edge, and face selection and manipulation
- **Scene & Object Management** — add, remove, select, and transform 3D objects
- **Material & Shading** — PBR materials with diffuse color, metallic, roughness
- **Import/Export** — Load OBJ files, save projects in `.junie3d` format
- **UI Panels** — Viewport, scene hierarchy, properties, materials, toolbar

## Architecture

Event-Driven ECS (Entity-Component-System) with:
- Entity IDs for scene objects
- Components: Transform, Mesh, Material, Name
- Event bus for decoupled system communication

## Building

```bash
cargo build --release
```

The binary will be at `target/release/junie3d`.

## Installation

Download the latest release from the [Releases page](https://github.com/junie3d/junie3d/releases).

### Windows
- **Installer**: `Junie3D-0.1.0-setup.exe`
- **Portable**: `junie3d.exe`

### Linux
- **Debian/Ubuntu**: `junie3d-linux-x86_64`
- **Arch Linux**: `junie3d-linux-x86_64` (same binary, dynamically linked)

## Tech Stack

- **UI**: egui + eframe (glow backend)
- **Math**: glam
- **Serialization**: serde + bincode
- **3D Formats**: tobj (OBJ)

## License

MIT
