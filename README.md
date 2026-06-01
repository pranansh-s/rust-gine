# Rust OpenGL 2D Game Engine

A modern, highly performant 2D game engine built with Rust and OpenGL. Features a clean, explicit resource ownership model, unified rendering pipelines, compile-safe DOM UI hierarchy, and high-precision timing.

---

## Core Features

- 🛠️ **Explicit Ownership Architecture**: Zero-global state model. `GameManager` owns the active subsystems, preventing thread races and ensuring deterministic lifecycles.
- 📷 **High-Performance Camera**: Zoom-to-cursor, panning, scaling, and translation view matrix adjustments.
- 🎨 **Unified Render Pipeline**: Highly optimized sprite drawing under a single `SpriteRenderer` abstraction.
- 🧩 **No-Leak Resource Management**: HashMaps indexed via owned `String` keys, removing unsafe `Box::leak` calls and ensuring safe resource recovery on exit.
- 🖌️ **Compile-Safe UI DOM**: Comment-free, dynamic owned UI tree hierarchy (`Div`) supporting easy child nesting and flexible property mappings.
- ⏱️ **Premium Native Timing**: Real-time elapsed counters, precise frame delta measurement, and repeating or one-shot callback task scheduling.
- 📊 **Resource Logging**: Live monitoring of engine memory footprint and system CPU usage.

---

## Project Structure

The project separates source code, executables, and game assets:

- **src/**: Core library engine modules.
- **examples/**: Subfolder-based clean code executables demonstrating different subsystems.
- **assets/**: Game assets, shaders, configs, and textures.
  - **assets/shaders/**: Terrain, Character, and UI shaders.
  - **assets/maps/**: Grid configuration files (`main.ini`).
  - **assets/*.png**: Textures (character, grass, and path).

---

## Getting Started

### Prerequisites

Ensure you have a graphics driver supporting OpenGL 3.3 core profiles.

```bash
# Verify cargo is installed and up-to-date
cargo --version
```

### Running the Subfolder Standalone Examples

The engine features three standalone, aptly named examples in different subfolders of `examples/`, showcasing distinct features of the engine using assets loaded via relative path traversal:

#### 1. Character & Sprite Rendering Demo
Demonstrates singular player movement mapping, keyboard controller boundaries, pacing, and sprite rendering.
```bash
cargo run --example character
```
*Controls*: **W, A, S, D** to move player.

#### 2. Terrain & Camera Drag-Panning Demo
Demonstrates loading map definitions from `assets/maps/main.ini` (which dynamic-registers terrain grass/path textures), grid cell rendering, and full drag-panning controls.
```bash
cargo run --example map
```
*Controls*: **Hold Left Click & Drag** to pan the camera; **Mouse Scroll** to zoom.

#### 3. DOM UI Rendering Demo
Demonstrates building structural DOM-inspired nested UI panel hierarchies and rendering them using custom shaders.
```bash
cargo run --example ui
```

---

## Running Engine Tests

To verify that the timing systems, scheduled events, and UI hierarchy are functioning perfectly:

```bash
cargo test
```