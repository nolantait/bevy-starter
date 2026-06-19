---
name: bevy-bsn
description: Reference for Bevy Scene Notation (BSN) — the bsn! macro for defining inline scenes with components, children, relationships, observers, and dynamic props.
metadata:
  crate: bevy_scene
  bevy: "0.18"
---

## Basic usage

`bsn!` spawns an entity with components inline:

```rust
#[derive(Component, Clone, Default)]
struct Ship;

#[derive(Component, Clone, Default)]
struct Player { score: usize }

fn spawn_scene(mut commands: Commands) {
  commands.spawn_scene(bsn! {
      Player
      Ship
  });
}
```

## Reusable scenes

Functions returning `impl Scene` compose together:

```rust
fn button() -> impl Scene {
  bsn! {
      Button
      Node { width: px(100) }
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, my_button.spawn())
    .run();
}
```

## Inline expressions as field values

Use `{expr}` for dynamic values:

```rust
fn increment_score(current_points: usize) -> impl Scene {
  bsn! {
      Player { score: {current_points + 10} }
  }
}
```

## Scene merging (component combination)

Two scenes defining the same component are merged field by field:

```rust
fn button() -> impl Scene {
  bsn! { Button Node { width: px(100) } }
}

fn my_button() -> impl Scene {
  bsn! {
      button()
      Node { height: px(100) }
  }
}
// my_button produces Node with both width: px(100) and height: px(100)
```

## Entity separation: commas vs whitespace

- **Whitespace**: components on the same entity
- **Commas**: separates entities

```rust
// One child with A and B
Children [ A B ]

// Two children, one with A, one with B
Children [ A, B ]

// Two children, clearer with parentheses
Children [ (A B), C ]
```

## SceneComponent derive

`#[derive(SceneComponent)]` aggregates components under a name with a `scene()` method:

```rust
#[derive(SceneComponent, Default, Clone)]
struct Car { boost: f32 }

impl Car {
  fn scene() -> impl Scene {
    bsn! {
      Transform { translation: Vec3 { x: 10. } }
      Children [
        FrontWheel,
        BackWheel,
      ]
    }
  }
}
```

Spawning with `@` prefix in `bsn!`:

```rust
fn spawn_car(mut commands: Commands) {
  commands.spawn_scene(bsn! {
    @Car { boost: 100. }
  });
}
```

## Dynamic props

Pass a config struct to customize scene contents:

```rust
#[derive(Default)]
struct CarConfig { wheels: WheelSize }

#[derive(Default)]
enum WheelSize { #[default] Standard, Wide }

fn car_with_config(config: CarConfig) -> impl Scene {
  let wheels: Box<dyn Scene> = match config.wheels {
    WheelSize::Standard => Box::new(bsn! { SlimWheels }),
    WheelSize::Wide => Box::new(bsn! { WideWheels }),
  };
  bsn! { #Car wheels }
}
```

## Named entity references

Prefix a `#` to name an entity (via its `Name` component), then reference it in the same scene:

```rust
#[derive(Component, FromTemplate)]
struct EmployedBy(Entity);

fn boss() -> impl Scene {
  bsn! {
      #Boss
      Children [
          #Joe EmployedBy(#Boss)
      ]
  }
}
```

Works in `bsn_list!` too:

```rust
fn employees() -> impl SceneList {
  bsn_list! [
      (#Joe ReportsTo(#Jane)),
      (#Jane ReportsTo(#Joe)),
  ]
}
```

## Observers inline

Attach observers with the `on` keyword:

```rust
fn button() -> impl Scene {
  bsn! {
      Node { width: px(100), height: px(50) }
      on(|press: On<Pointer<Press>>| {
          info!("button pressed!")
      })
  }
}
```

## Children

Use `Children` to nest entities:

```rust
fn spawn_scene(mut commands: Commands) {
  commands.spawn_scene(bsn! {
    Player
    Children [
      Sword,
      Shield,
    ]
  });
}
```

Also works with custom relationships via the `relationship` or `related!` APIs (see [relationships skill](/bevy-relationships)).
