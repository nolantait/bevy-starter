---
name: bevy-cameras
description: Reference for cameras in Bevy — Camera2d, Camera3d, viewports, projection, render layers, mouse-to-world, and free camera controllers.
metadata:
  crate: bevy_camera
  bevy: "0.19"
---

## Creating cameras

```rust
commands.spawn(Camera2d);
commands.spawn(Camera3d);
```

Mark a main camera with a marker component for querying:

```rust
#[derive(Component)]
#[require(Camera2d)]
struct MainCamera;
```

## Camera = render target + projection + position

Each `Camera` defines:
1. A `RenderTarget` (Window, Image, TextureView, or None)
2. A `Projection` (orthographic for 2D, perspective for 3D)
3. Position via `Transform`

## Viewports

Multiple cameras can render to the same window via viewports (split-screen, minimap):

```rust
commands.spawn((Camera2d, Camera { viewport: Some(Viewport { .. }), ..default() }));
```

## Coordinate system

Right-handed: X→right, Y→up, Z→coming out of the screen toward the viewer
("forward" is -Z). Default screen center is (0,0).

## 2D camera movement

```rust
fn move_camera(time: Res<Time>, input: Res<ButtonInput<KeyCode>>, mut t: Single<&mut Transform, With<MainCamera>>) {
  let mut dir = Vec3::ZERO;
  if input.pressed(KeyCode::KeyW) { dir.y += 1.; }
  if input.pressed(KeyCode::KeyS) { dir.y -= 1.; }
  if input.pressed(KeyCode::KeyA) { dir.x -= 1.; }
  if input.pressed(KeyCode::KeyD) { dir.x += 1.; }
  if dir != Vec3::ZERO { t.translation += dir.normalize() * time.delta_secs() * 500.; }
}
```

Zoom: modify `Projection::Orthographic.scale`.

## 3D camera movement

```rust
fn move_3d(input: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut t: Single<&mut Transform, With<MainCamera>>) {
  let mut dir = Vec3::ZERO;
  if input.pressed(KeyCode::KeyW) { dir += *t.forward(); }
  if input.pressed(KeyCode::KeyS) { dir -= *t.forward(); }
  if input.pressed(KeyCode::KeyA) { dir -= *t.right(); }
  if input.pressed(KeyCode::KeyD) { dir += *t.right(); }
  if dir != Vec3::ZERO { t.translation += dir.normalize() * time.delta_secs() * 10.; }
}
```

## 3D mouse look

```rust
use bevy::input::mouse::MouseMotion;

fn look(mut mm: MessageReader<MouseMotion>, mut t: Single<&mut Transform, With<Camera>>) {
  let mouse_sensitivity = Vec2::new(0.12, 0.10);

  for motion in mm.read() {
    let delta_yaw = -motion.delta.x * mouse_sensitivity.x;
    let delta_pitch = -motion.delta.y * mouse_sensitivity.y;

    // Yaw turns left/right (global)
    t.rotate_y(delta_yaw);

    // Pitch looks up/down (local), clamped to the poles
    const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
    let (yaw, pitch, roll) = t.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
    t.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
  }
}
```

Zoom (3D): modify `Projection::Perspective.fov`.

## Built-in camera controllers

Enable `free_camera` feature. Then:

```rust
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
```

## Render layers

```rust
use bevy::camera::visibility::RenderLayers;
const BG: RenderLayers = RenderLayers::layer(1);
const FG: RenderLayers = RenderLayers::layer(2);

commands.spawn((FG, MainCamera));  // camera sees FG
commands.spawn((Player, FG));      // entity on FG
```

## Rendering order

Cameras with higher `Camera::order` render later (on top). Use `ClearColorConfig::None` to not clear:

```rust
commands.spawn((Camera3d::default(), Camera { order: 1, clear_color: ClearColorConfig::None, ..default() }));
```

## Mouse to world coordinates (2D)

```rust
fn mouse_to_world(window: Single<&Window>, camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>) {
  let (cam, gt) = camera.into_inner();
  if let Some(cursor) = window.cursor_position() {
    if let Ok(ray) = cam.viewport_to_world(gt, cursor) {
      let world = ray.origin.truncate();
      // use world.x, world.y
    }
  }
}
```
