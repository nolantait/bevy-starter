---
name: bevy-windows
description: Reference for Bevy windows — Window component, physical vs logical size, resolution, cursors, multi-window, events, screenshots, and reactive rendering.
metadata:
  crate: bevy_window
  bevy: "0.19"
---

## Window component

The `Window` component holds all window settings (position, size, title, etc.). The `PrimaryWindow` marker is added to the default window.

```rust
fn log(window: Single<&Window>) {
  info!("width: {}, height: {}", window.width(), window.height());
}
```

## Physical vs logical size

- **Physical size** — actual pixels on monitor (unscaled)
- **Logical size** — physical / `scale_factor`
- **Requested size** — logical size submitted to API

## Configuring the primary window

```rust
app.add_plugins(DefaultPlugins.set(WindowPlugin {
  primary_window: Some(Window {
    title: "My Game".into(),
    resolution: WindowResolution::new(500, 300).with_scale_factor_override(1.0),
    present_mode: PresentMode::AutoVsync,
    ..default()
  }),
  ..default()
}));
```

## Querying windows

```rust
fn inspect(windows: Query<&Window, With<PrimaryWindow>>) {
  for window in &windows {
    let logical = (window.width(), window.height());
    let physical = (window.physical_width(), window.physical_height());
    let cursor = window.cursor_position(); // Option<Vec2>
  }
}
```

## Changing resolution at runtime

```rust
fn resize(mut window: Single<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
  if keys.just_pressed(KeyCode::Digit1) {
    window.resolution.set(800., 600.);
  }
}
```

## Changing title

```rust
fn title(mut windows: Query<&mut Window>, time: Res<Time>) {
  if let Ok(mut window) = windows.single_mut() {
    window.title = format!("FPS: {:.0}", 1.0 / time.delta_secs());
  }
}
```

## Cursor

```rust
use bevy::window::{CursorGrabMode, CursorOptions};

fn toggle_cursor(mut cursor: Single<&mut CursorOptions, With<Window>>, input: Res<ButtonInput<KeyCode>>) {
  if input.just_pressed(KeyCode::Space) {
    cursor.visible = !cursor.visible;
    cursor.grab_mode = match cursor.grab_mode {
      CursorGrabMode::None => CursorGrabMode::Locked,
      _ => CursorGrabMode::None,
    };
  }
}
```

## Closing on Escape

```rust
pub fn close_on_esc(mut commands: Commands, windows: Query<(Entity, &Window)>, input: Res<ButtonInput<KeyCode>>) {
  for (window, _) in windows.iter().filter(|(_, w)| w.focused) {
    if input.just_pressed(KeyCode::Escape) {
      commands.entity(window).despawn();
    }
  }
}
```

## Window events

`WindowEvent` enum covers: `WindowResized`, `WindowMoved`, `WindowFocused`, `WindowCloseRequested`, `WindowCreated`, `WindowDestroyed`, `CursorMoved`, `KeyboardInput`, `MouseWheel`, and more.

## Reactive windows (power saving)

```rust
app.insert_resource(WinitSettings {
  focused_mode: UpdateMode::Continuous,
  unfocused_mode: UpdateMode::reactive_low_power(Duration::from_millis(10)),
});
```

## Screenshot

```rust
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

fn take_screenshot(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
  if input.just_pressed(KeyCode::Space) {
    commands.spawn(Screenshot::primary_window()).observe(save_to_disk("screenshot.png"));
  }
}
```
