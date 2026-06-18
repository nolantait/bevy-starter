---
name: bevy-plugins
description: Reference for Bevy plugins — function plugins, Plugin trait, life-cycle, ordering, configuration, PluginGroup, DefaultPlugins, and MinimalPlugins.
metadata:
  crate: bevy_app
  bevy: "0.18"
---

## Function plugin

```rust
fn my_plugin(app: &mut App) {
  app.add_systems(Startup, setup);
}
app.add_plugins(my_plugin);
```

## Plugin trait

```rust
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
  fn cleanup(&self, _app: &mut App) {
    // clean up resources
  }
}
```

Lifecycle: `build` → `ready` (polled) → `cleanup`.

## Plugin ordering and dependencies

Plugins run in the order added. Duplicate plugins panic by default:

```rust
if !app.is_plugin_added::<MyPlugin>() {
  app.add_plugins(MyPlugin);
}
```

## Plugin configuration

```rust
pub struct CameraPlugin { debug: bool }

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    if self.debug { /* extra setup */ }
  }
}
app.add_plugins(CameraPlugin { debug: true });
```

## Plugin groups

Group related plugins and allow later configuration:

```rust
pub struct GamePlugins;
impl PluginGroup for GamePlugins {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(CameraPlugin::default())
      .add(PhysicsPlugin::default())
  }
}

app.add_plugins(GamePlugins.build().disable::<PhysicsPlugin>());
```

## DefaultPlugins

Everything needed for a typical Bevy app — windowing, input, audio, rendering, UI, scenes, assets, time, transforms, etc. (feature-gated, enabled by default).

## MinimalPlugins

Bare essentials: `TaskPoolPlugin`, `FrameCountPlugin`, `TimePlugin`, `ScheduleRunnerPlugin`. Useful for tests and headless apps.
