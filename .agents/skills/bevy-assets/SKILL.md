---
name: bevy-assets
description: Reference for loading, managing, and tracking assets in Bevy — AssetServer, handles, loading states, events, custom loaders, and hot reloading.
metadata:
  crate: bevy_asset
  bevy: "0.19"
---

## Core resources

- `Assets<T>` — stores loaded assets of each type
- `AssetServer` — loads assets from files asynchronously

```rust
fn load(mut image: ResMut<MyImage>, server: Res<AssetServer>) {
  image.0 = server.load("images/foo.png");
}
```

## Creating assets procedurally

```rust
fn spawn(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
  let mesh = meshes.add(Circle::new(50.));
  let material = materials.add(Color::BLACK);
  commands.spawn((Mesh2d(mesh), MeshMaterial2d(material)));
}
```

## Handle types by extension

| Extension | Handle |
|-----------|--------|
| `ttf`, `otf` | `Font` |
| `png`, `jpg`, `webp`, `ktx2`, `bmp`, `gif`, `tga`, `tiff` | `Image` |
| `mp3`, `flac`, `ogg`, `wav` | `AudioSource` |
| `gltf`, `glb` | `Gltf` (use `GltfAssetLabel::Scene(0)` for a scene) |
| `scn`, `scn.ron` | `DynamicWorld` |

## Loading 3D models

```rust
let scene: Handle<Scene> = server.load(GltfAssetLabel::Scene(0).from_asset("models/car.glb"));
commands.spawn((SceneRoot(scene), Transform::default()));
```

## Loading state

```rust
use bevy::asset::LoadState;
match server.get_load_state(&handle) {
  Some(LoadState::Loaded) => {},
  Some(LoadState::Loading) => {},
  Some(LoadState::Failed(_)) => {},
  _ => {}
}
```

## Asset events

```rust
use bevy::asset::AssetEvent;
fn react(mut events: MessageReader<AssetEvent<Image>>) {
  for event in events.read() {
    match event {
      AssetEvent::Added { id } => {},
      AssetEvent::LoadedWithDependencies { id } => {},
      AssetEvent::Modified { id } => {},
      AssetEvent::Removed { id } => {},
      AssetEvent::Unused { id } => {},
    }
  }
}
```

## Loading a folder

```rust
use bevy::asset::LoadedFolder;
let _folder: Handle<LoadedFolder> = server.load_folder("models/monkey");
```

## Custom asset source

```rust
app.register_asset_source("custom", AssetSourceBuilder::platform_default("assets/custom", None));
// Then: server.load("custom://file.png")
```

## Custom asset loader

```rust
#[derive(Asset, TypePath)]
struct TextFile { content: String }

impl AssetLoader for TextFile {
  type Asset = TextFile; type Settings = (); type Error = std::io::Error;
  async fn load(&self, reader: &mut dyn Reader, _settings: &(), _ctx: &mut LoadContext<'_>) -> Result<Self::Asset, Self::Error> {
    let mut content = String::new();
    reader.read_to_string(&mut content).await?;
    Ok(TextFile { content })
  }
  fn extensions(&self) -> &[&str] { &["txt"] }
}
// Then: app.init_asset::<TextFile>()
```

## Hot reloading

Enable via feature `file_watcher` in Cargo.toml, or:

```rust
app.add_plugins(DefaultPlugins.set(AssetPlugin {
  watch_for_changes_override: Some(true),
  ..default()
}));
```

## Handle lifecycle

- Cloning a `Handle<T>` creates a strong reference
- Assets are unloaded when all strong handles are dropped
- `Handle::Weak` does not keep the asset alive
