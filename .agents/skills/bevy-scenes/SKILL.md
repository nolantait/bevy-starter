---
name: bevy-scenes
description: Reference for Bevy scenes — saving and loading entity/component snapshots via reflection, DynamicWorld, WorldInstanceSpawner, and DynamicWorldRoot.
metadata:
  crate: bevy_scene
  bevy: "0.19"
---

## Scene types

- `DynamicWorld` — serializable representation (no own world), stores `Vec<Box<dyn PartialReflect>>` resources + `Vec<DynamicEntity>` entities

## Saving a scene

```rust
fn save_scene(world: &mut World) {
  let scene = DynamicWorld::from_world(world);
  let type_registry = world.resource::<AppTypeRegistry>();
  let reg = type_registry.read();
  let serialized = scene.serialize(&reg).unwrap();
  // write serialized to file (e.g., "scene.scn.ron")
}
```

## Loading a scene

Three ways:

```rust
// 1. DynamicWorldRoot component
fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
  commands.spawn(DynamicWorldRoot(asset_server.load("scene.scn.ron")));
}

// 2. WorldInstanceSpawner::spawn_dynamic
// 3. DynamicWorldBuilder
```

## Scene events

`WorldInstanceReady` fires when the scene is fully loaded. `SceneInstance` component is added to the root entity:

```rust
fn on_loaded(mut spawner: ResMut<WorldInstanceSpawner>, query: Query<&SceneInstance>) {
  for instance in &query {
    spawner.despawn_instance_sync(world, instance);
  }
}
```

## Serialization format (RON)

```scn
(
  resources: { "my::Resource": (score: 1) },
  entities: {
    4294967297: (
      components: {
        "bevy_transform::components::transform::Transform": (
          translation: (0.0, 0.0, 0.0),
          rotation: (0.0, 0.0, 0.0, 1.0),
          scale: (1.0, 1.0, 1.0),
        ),
      },
    ),
  },
)
```

## Required setup

- Components must `#[derive(Reflect)]` for auto-registration
- `FromWorld` on a component customizes initialization during load
- `#[reflect(skip_serializing)]` excludes fields from serialization
