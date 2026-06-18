---
name: bevy-gizmos
description: Reference for Bevy gizmos — immediate-mode visual debugging, GizmoPlugin, Gizmos system param, retained GizmoAsset, config groups, and line styles.
metadata:
  crate: bevy_gizmos
  bevy: "0.18"
---

## Setup

Gizmos are enabled via `GizmoPlugin`, which is included in `DefaultPlugins`:

```rust
App::new()
    .add_plugins(DefaultPlugins)  // includes GizmoPlugin
    .run();
```

Or add it manually: `.add_plugins(GizmoPlugin)`.

## Immediate mode — `Gizmos` system param

Gizmos are drawn per-frame and cleared automatically after rendering.

```rust
fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, GREEN);
}
```

Use in `Update` (per-frame) or `FixedMain` (per-tick).

## Drawing primitives

| Shape | 3D | 2D |
|-------|----|----|
| Line | `gizmos.line(start, end, color)` | `gizmos.line_2d(start, end, color)` |
| Ray | `gizmos.ray(origin, direction, color)` | `gizmos.ray_2d(origin, direction, color)` |
| Linestrip | `gizmos.linestrip(positions, color)` | `gizmos.linestrip_2d(positions, color)` |
| Lineloop | `gizmos.lineloop(positions, color)` | — |
| Rect | `gizmos.rect(isometry, size, color)` | `gizmos.rect_2d(isometry, size, color)` |
| Circle | `gizmos.circle(isometry, radius, color)` | `gizmos.circle_2d(isometry, radius, color)` |
| Sphere | `gizmos.sphere(isometry, radius, color)` | — |
| Ellipse | `gizmos.ellipse(isometry, half_size, color)` | `gizmos.ellipse_2d(isometry, half_size, color)` |
| Cube | `gizmos.cube(transform, color)` | — |
| Arrow | `gizmos.arrow(start, end, color)` | `gizmos.arrow_2d(start, end, color)` |
| Cross | `gizmos.cross(isometry, half_size, color)` | `gizmos.cross_2d(isometry, half_size, color)` |
| Grid | `gizmos.grid(rotation, cells, spacing, color)` | `gizmos.grid_2d(...)` |
| AABB | `gizmos.aabb_3d(aabb, transform, color)` | — |

### Lines with color gradients

```rust
gizmos.line_gradient(Vec3::ZERO, Vec3::X, GREEN, RED);
gizmos.ray_gradient(Vec3::Y, Vec3::X, CYAN, MAGENTA);
gizmos.linestrip_gradient([(Vec3::ZERO, GREEN), (Vec3::X, RED)]);
```

### Curves

Requires `bevy_math::curve::Curve` trait:

```rust
let domain = Interval::UNIT;
let curve = FunctionCurve::new(domain, |t| Vec2::from(t.sin_cos()));
gizmos.curve_2d(curve, (0..=100).map(|n| n as f32 / 100.0), RED);
gizmos.curve_3d(curve_3d, times, BLUE);

// With gradient:
gizmos.curve_gradient_2d(curve, (0..=100).map(|n| n as f32 / 100.0).map(|t| (t, GREEN.mix(&RED, t))));
```

### Arcs & ellispes

Arc builder methods return a builder with `.resolution(n)`:

```rust
gizmos.arc_3d(angle_rad, radius, isometry, color).resolution(64);
gizmos.short_arc_3d_between(center, from, to, color);
gizmos.long_arc_3d_between(center, from, to, color);
```

### Arrow builder

```rust
gizmos.arrow(start, end, ORANGE_RED)
    .with_double_end()
    .with_tip_length(0.5);
```

### Axes

Draw XYZ axes from a transform:

```rust
gizmos.axes(transform, base_length);
gizmos.axes_2d(transform, base_length);
```

### Rounded cuboids / rectangles

```rust
gizmos.rounded_cuboid(center, size, TURQUOISE)
    .edge_radius(0.1)
    .arc_resolution(4);
gizmos.rounded_rect(isometry, size, color);
gizmos.rounded_rect_2d(isometry, size, color);
```

### Primitives

All `bevy_math` primitives renderable:

```rust
gizmos.primitive_3d(
    &Plane3d { normal: Dir3::Y, half_size: Vec2::splat(1.0) },
    isometry,
    GREEN,
).cell_count(UVec2::new(5, 10))
 .spacing(Vec2::new(0.2, 0.1));
```

## Config groups

Create custom config groups to independently toggle/style sets of gizmos:

```rust
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos;

app.init_gizmo_group::<MyRoundGizmos>();

fn system(mut my_gizmos: Gizmos<MyRoundGizmos>) {
    my_gizmos.sphere(Isometry3d::IDENTITY, 1.0, RED);
}
```

Use `AppGizmoBuilder::insert_gizmo_config` for custom initial config.

## Config store (`GizmoConfigStore`)

Access and modify gizmo configuration at runtime:

```rust
fn update_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled ^= true;  // toggle visibility
    config.depth_bias = -1.0;  // always in front
    config.line.width = 5.0;
    config.line.perspective ^= true;

    // Custom config group
    let (my_config, _) = config_store.config_mut::<MyRoundGizmos>();
    my_config.line.width += 1.0;
}
```

### Config fields

| Field | Type | Description |
|-------|------|-------------|
| `enabled` | `bool` | Toggle all gizmos for this group |
| `depth_bias` | `f32` | -1 (in front) to 1 (behind); 0 = normal |
| `line.width` | `f32` | Line width in pixels |
| `line.perspective` | `bool` | Perspective-correct line width |
| `line.style` | `GizmoLineStyle` | `Solid`, `Dotted`, `Dashed { gap_scale, line_scale }` |
| `line.joints` | `GizmoLineJoint` | `Bevel`, `Miter`, `Round(n)`, `None` |
| `mesh` | `MeshConfig` | Mesh gizmo config |

## AABB gizmos

```rust
// Enable on specific entities by adding ShowAabbGizmo component
// Or draw all with:
config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all = true;
```

## Retained mode (`GizmoAsset` + `Gizmo` component)

For many static lines, use retained gizmos for better performance:

```rust
fn spawn_retained(mut commands: Commands, mut gizmo_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmo = GizmoAsset::new();
    gizmo.sphere(Isometry3d::IDENTITY, 0.5, CRIMSON)
        .resolution(30_000 / 3);

    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmo),
        line_config: GizmoLineConfig { width: 5.0, ..default() },
        ..default()
    });
}
```

`Gizmo` component fields: `handle` (Handle\<GizmoAsset\>), `line_config`, `depth_bias`.

`GizmoAsset` supports the same drawing API as `Gizmos` (line, circle, sphere, etc.).

## Light gizmos

Feature: `bevy_light`. Debug visualization of lights:

```rust
// Requires bevy_light feature
// Automatically drawn for PointLight, SpotLight, DirectionalLight
```
