---
name: bevy-resources
description: Reference for Bevy resources — defining, inserting, initializing, reading/writing, and removing global singleton data.
metadata:
  crate: bevy_ecs
  bevy: "0.19"
---

## Defining a resource

```rust
#[derive(Resource)]
struct Score(usize);
```

Only one per type per `World`.

## Adding resources

```rust
// With an instance
app.insert_resource(Score(0));

// With Default or FromWorld
app.init_resource::<Score>();
```

Dynamic insertion from a system:

```rust
commands.insert_resource(Score(0));
```

## Removing resources

```rust
commands.remove_resource::<Score>();
```

## Reading and writing

```rust
fn read(score: Res<Score>) { }
fn write(mut score: ResMut<Score>) {
  score.0 += 1;
}
```

## Optional resources

Avoid panics when resource might not exist:

```rust
fn maybe_score(mut score: Option<ResMut<Score>>) {
  if let Some(mut score) = score.as_deref_mut() {
    score.0 += 1;
  }
}
```

Prefer `init_resource` at app definition over dynamic creation to ensure availability.
