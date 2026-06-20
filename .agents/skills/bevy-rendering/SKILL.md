---
name: bevy-rendering
description: Reference for Bevy's rendering pipeline — wgpu, render graph, extract/prepare/queue/draw stages, meshes, materials, lighting, and text rendering.
metadata:
  crate: bevy_render
  bevy: "0.19"
---

## Graphics stack

```
Bevy Engine → wgpu → Graphics API (Vulkan/Metal/DX12/WebGPU) → GPU Driver → GPU Hardware
```

## Frame structure

Each frame: **Simulation** (game logic) + **Rendering** (drawing) run in parallel.

## Render pipeline (5 steps)

1. **Extract** — copy render data from game world (sync point, keep fast)
2. **Prepare** — set up vertex data, write vertex buffers
3. **Queue** — create pipelines, bind groups, add entities to render phases
4. **Render Graph** — acyclic graph of nodes generating GPU commands
5. **Draw Functions** — execute draw calls via `RenderCommand`

## Render Graph

- `Nodes` — generate draw calls, operate on slots
- `Edges` — execution order, connect input/output slots
- `Slots` — describe render resources

Supports sub-graphs (e.g., "2d" and "3d" for different parts of the game).

## Meshes

Built-in 3D: `Cuboid`, `Sphere`, `Cylinder`, `Capsule3D`, `Cone`, `Torus`, `Triangle3D`, `Tetrahedron`, `Plane3D`

Built-in 2D: `Circle`, `Rectangle`, `Triangle2d`, `Capsule2d`, `Ellipse`, `RegularPolygon`, `Arc2d`, and more.

## Materials

PBR properties: Color, Metallic, Roughness, Reflectance, Clear Coat, Anisotropy.

## Rendering entities

```rust
commands.spawn((
  Mesh2d(meshes.add(Circle::new(50.))),
  MeshMaterial2d(materials.add(ColorMaterial::from(RED))),
  Transform::from_xyz(-150., 0., 0.),
));
```

## Lighting

```rust
commands.spawn((
  PointLight { shadows_enabled: true, intensity: 10_000_000., range: 100.0, ..default() },
  Transform::from_xyz(0., 10., 0.),
));
```

Types: `PointLight`, `SpotLight`, `DirectionalLight`.

## Text rendering

UI text (`Text`) or scene text (`Text2d`):

```rust
// Scene text
commands.spawn((
  Text2d::new("Hello"),
  TextColor(Color::WHITE),
  TextFont { font_size: 60.0, ..default() },
  TextLayout::new_with_justify(Justify::Center),
));
```
