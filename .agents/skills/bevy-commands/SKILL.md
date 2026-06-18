---
name: bevy-commands
description: Reference for Bevy Commands — spawning/despawning entities, inserting/removing components, custom commands, trait extensions, and testing.
metadata:
  crate: bevy_ecs
  bevy: "0.18"
---

## Basics

`Commands` is a system parameter that queues mutations. They execute together at the next schedule transition (via `ApplyDeferred`).

```rust
fn spawn(mut commands: Commands) {
  commands.spawn_empty();
}
```

## Spawning components

```rust
commands.spawn(Player).insert(Transform::default());
// or with a tuple bundle
commands.spawn((Player, Transform::from_xyz(1., 1., 1.)));
```

## Required components

```rust
#[derive(Component)]
#[require(Transform)]
struct Player;
// now: commands.spawn(Player) auto-adds Transform
```

## EntityCommands chaining

`spawn`/`spawn_empty` return `EntityCommands` for chaining:

```rust
commands
  .spawn(Player)
  .insert(Transform::default())
  .insert(Name::new("Player 1"));
```

## Despawning

```rust
for entity in &query {
  commands.entity(entity).despawn();
}
```

## Commands are deferred

Commands don't execute immediately — they're queued until the next `ApplyDeferred` system runs (typically at schedule boundaries).

## Custom commands

Implement the `Command` trait:

```rust
struct SpawnPlanet { radius: f32, position: Vec2 }

impl Command for SpawnPlanet {
  fn apply(self, world: &mut World) {
    let mesh = world.resource_scope(|_w, mut meshes: Mut<Assets<Mesh>>| {
      meshes.add(Circle::new(self.radius))
    });
    // ...
    world.spawn((Planet::new(self.radius), Mesh2d(mesh), Transform::from_translation(self.position.extend(0.))));
  }
}
// usage:
commands.queue(SpawnPlanet { radius: 100., position: Vec2::ZERO });
```

## Extending the Commands API

```rust
trait MyExt {
  fn do_thing(&mut self);
}
impl<'w, 's> MyExt for Commands<'w, 's> {
  fn do_thing(&mut self) { info!("thing done"); }
}
```

## Testing commands

```rust
use bevy::ecs::world::CommandQueue;

let mut world = World::default();
let mut queue = CommandQueue::default();
queue.push(MyCommand);
queue.apply(&mut world);
```
