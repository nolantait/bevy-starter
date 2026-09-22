---
name: bevy-plugins
description: Reference for Bevy plugins — function plugins, Plugin trait, life-cycle, ordering, configuration, PluginGroup, DefaultPlugins, and MinimalPlugins.
metadata:
  crate: bevy_app
  bevy: "0.19"
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
  fn finish(&self, _app: &mut App) {
    info!("Camera plugin finished setting up");
  }
}
```

Lifecycle: `build` → `finish` → `ready` (polled) → `cleanup`. Plugins are built
in the order they are added to the `App`.

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

Everything needed for a typical Bevy app — windowing, input, audio, rendering,
UI, scenes, assets, time, transforms, etc. (feature-gated, enabled by default):

| Plugin | Feature | Description |
| --- | --- | --- |
| **DiagnosticsPlugin** |  | Adds core diagnostics |
| **FrameCountPlugin** |  | Adds frame counting functionality |
| **InputPlugin** |  | Adds keyboard and mouse input |
| **PanicHandlerPlugin** |  | Adds sensible panic handling |
| **TaskPoolPlugin** |  | Setup of default task pools for multi-threading |
| **TimePlugin** |  | Adds time functionality |
| **TransformPlugin** |  | Handles `Transform` components |
| **AccessibilityPlugin** | `bevy_window` | Adds non-GUI accessibility functionality |
| **AnimationPlugin** | `bevy_animation` | Adds animation support |
| **AntiAliasPlugin** | `bevy_anti_alias` | Adds multi-sample anti-aliasing (MSAA) |
| **AssetPlugin** | `bevy_asset` | Adds asset server and resources to load assets |
| **AudioPlugin** | `bevy_audio` | Adds support for using sound assets |
| **CameraPlugin** | `bevy_camera` | Adds 2D and 3D camera components and systems |
| **CiTestingPlugin** | `bevy_ci_testing` | Helps instrument continuous integration |
| **ClipboardPlugin** | `bevy_clipboard` | Adds clipboard support to a Bevy app |
| **CorePipelinePlugin** | `bevy_core_pipeline` | The core rendering pipeline |
| **DefaultPickingPlugins** | `bevy_picking` | Adds picking functionality |
| **DlssInitPlugin** | `dlss` | Initializes DLSS support if available |
| **GilrsPlugin** | `bevy_gilrs` | Adds support for gamepad inputs |
| **GizmoPlugin** | `bevy_gizmos` | Provides an immediate mode drawing api for visual debugging |
| **GizmoRenderPlugin** | `bevy_gizmos_render` | Sends gizmos to the renderer |
| **GltfPlugin** | `bevy_gltf` | Adds support for loading gltf models |
| **HotPatchPlugin** | `hotpatching` | Enables hot-patching of assets |
| **ImagePlugin** | `bevy_image` | Adds the `Image` asset and prepares them to render on your GPU |
| **InputDispatchPlugin** | `bevy_input_focus` | Dispatches bubbling keyboard and gamepad button events to the focused entity |
| **InputFocusPlugin** | `bevy_input_focus` | Sets up the core input focus system |
| **LightPlugin** | `bevy_light` | Adds light components and systems |
| **LogPlugin** | `bevy_log` | Adds logging to apps |
| **MeshPlugin** | `bevy_mesh` | Adds the `Mesh` asset and prepares them to render on the GPU |
| **PbrPlugin** | `bevy_pbr` | Adds physical based rendering with `StandardMaterial` etc |
| **PipelinedRenderingPlugin** | `bevy_render` | Adds pipelined rendering |
| **PostProcessPlugin** | `bevy_post_process` | Adds post processing effects |
| **RenderDebugOverlayPlugin** | `bevy_dev_tools` + `bevy_pbr` | Adds a rendering debug overlay to visualize various renderer buffers |
| **RenderPlugin** | `bevy_render` | Sets up rendering backend powered by `wgpu` crate |
| **ScenePlugin** | `bevy_scene` | Adds support for spawning Bevy Scenes |
| **ScheduleRunnerPlugin** | `!bevy_window` | Configures an `App` to run its `Schedule` according to a given `RunMode` (added when windowing is disabled) |
| **SpritePlugin** | `bevy_sprite` | Handling of sprites (images on our entities) |
| **SpriteRenderPlugin** | `bevy_sprite_render` | Adds support for sending sprites to the renderer |
| **StatesPlugin** | `bevy_state` | Adds state management for Apps |
| **TerminalCtrlCHandlerPlugin** | `std` | Handles Ctrl-C signals in terminal applications |
| **TextPlugin** | `bevy_text` | Supports loading fonts and rendering text |
| **UiPlugin** | `bevy_ui` | Adds support for UI layouts (flex, grid, etc) |
| **UiRenderPlugin** | `bevy_ui_render` | Adds support for sending UI nodes to renderer |
| **UiWidgetsPlugins** | `bevy_ui_widgets` | Registers the observers for all of the widgets in this crate |
| **WebAssetPlugin** | `bevy_asset` | Adds the `http` and `https` asset sources to an app |
| **WindowPlugin** | `bevy_window` | Provides an interface to create and manage `Window` components |
| **WinitPlugin** | `bevy_winit` | Interface to create operating system windows (to actually display our game) |
| **WorldSerializationPlugin** | `bevy_world_serialization` | Loading and saving collections of entities and components to files |

## MinimalPlugins

Bare essentials: `TaskPoolPlugin`, `FrameCountPlugin`, `TimePlugin`, `ScheduleRunnerPlugin`. Useful for tests and headless apps.
