---
name: bevy-enhanced-input
description: Reference for bevy_enhanced_input — observer-based input manager with actions, contexts, bindings, conditions, modifiers, presets, and mocking.
metadata:
  crate: bevy_enhanced_input
  bevy: "0.19"
---

## Setup

```toml
[dependencies]
bevy_enhanced_input = "0.26"
```

```rust
use bevy_enhanced_input::prelude::*;

App::new()
    .add_plugins(EnhancedInputPlugin)
    .add_input_context::<Player>()
    .finish();
```

## Core concepts

Three pillars: **Actions**, **Bindings**, **Contexts**.

- **Actions** — what the player can do (Jump, Move, Fire). Defined as structs with `#[derive(InputAction)]`.
- **Bindings** — which physical inputs trigger an action (keyboard keys, mouse, gamepad).
- **Contexts** — group of actions active in a given gameplay state (OnFoot, InVehicle). Registered via `add_input_context`.

## Defining an action

```rust
#[derive(InputAction)]
#[action_output(bool)]
struct Jump;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Movement;

#[derive(InputAction)]
#[action_output(f32)]
struct Zoom;
```

Output types: `bool`, `f32`, `Vec2`, `Vec3`.

## Spawning a context with actions and bindings

```rust
#[derive(Component)]
struct Player;

let mut app = App::new();
app.add_plugins(EnhancedInputPlugin)
    .add_input_context::<Player>()
    .finish();

app.world_mut().spawn((
    Player,
    actions!(Player[
        (
            Action::<Jump>::new(),
            bindings![KeyCode::Space, GamepadButton::South],
        ),
        (
            Action::<Movement>::new(),
            bindings![KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD],
        ),
    ]),
));
```

## Reacting to actions (push-style via observers)

```rust
fn on_jump(trigger: On<Fire<Jump>>, mut q: Query<&mut Transform>) {
    let mut t = q.get_mut(trigger.context).unwrap();
    t.translation.y += 5.0;
}

fn on_movement(trigger: On<Fire<Movement>>, mut q: Query<&mut Transform>) {
    let mut t = q.get_mut(trigger.context).unwrap();
    t.translation += trigger.value.extend(0.0) * 60.0 * trigger.time.delta_secs();
}

app.add_observer(on_jump).add_observer(on_movement);
```

Event types: `Start<A>` (began), `Fire<A>` (ongoing), `Ongoing<A>` (active but not fired), `Complete<A>` (ended), `Cancel<A>` (interrupted).

## Reacting to actions (pull-style via queries)

```rust
fn poll_input(
    jump: Single<&ActionEvents, With<Action<Jump>>>,
    movement: Single<&Action<Movement>>,
    mut t: Single<&mut Transform, With<Player>>,
) {
    if jump.contains(ActionEvents::STARTED) {
        t.translation.y += 5.0;
    }
    t.translation += movement.extend(0.0) * 60.0;
}
```

`Action<A>` holds the current value. `TriggerState` tracks the state enum. `ActionEvents` (bitfield) detects transitions. `ActionTime` has timing info.

## Input conditions

Conditions control *when* an action triggers. Attached to actions or bindings.

```rust
actions!(Player[
    (
        Action::<Jump>::new(),
        Hold::new(1.0),              // must hold for 1s
        bindings![KeyCode::Space],
    ),
    (
        Action::<Fire>::new(),
        Pulse::new(0.5),             // fires every 0.5s while held
        bindings![
            (GamepadButton::RightTrigger2, Down::new(0.3)), // threshold 0.3
            MouseButton::Left,
        ],
    ),
]);
```

Available conditions:
- `Down` — active while pressed (default with zero actuation threshold)
- `Press` — triggers on press only
- `Release` — triggers on release only
- `Hold` — triggers after held for N seconds
- `HoldAndRelease` — triggers on release after held for N seconds
- `Tap` — press + release within a time window
- `Pulse` — repeats every N seconds while held
- `Toggle` — toggles on/off each press
- `Chord` — requires multiple simultaneous inputs
- `Combo` — requires sequential inputs in order
- `Cooldown` — prevents re-triggering for N seconds
- `Flick` — triggers on quick directional input
- `BlockBy` — suppresses action while another action is firing

## Input modifiers

Modifiers transform the raw input value. Attached to actions or bindings.

```rust
actions!(Player[
    (
        Action::<Movement>::new(),
        DeadZone::default(),       // ignore small analog values
        SmoothNudge::default(),    // smoothing
        DeltaScale::default(),     // multiply by delta time
        Scale::splat(30.0),        // constant multiplier
        bindings![KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD],
    ),
]);
```

Available modifiers:
- `Clamp` — clamp value to range
- `DeadZone` — ignore near-zero values from analog inputs
- `DeltaScale` — scale by delta time for frame-rate independence
- `ExponentialCurve` — apply exponential response curve
- `LinearStep` — stepped response
- `Negate` — invert value
- `Scale` — multiply by constant
- `SmoothNudge` — smoothing
- `SwizzleAxis` — reorder axis mapping (e.g., map mouse scroll Y to action X)
- `AccumulateBy` — accumulate over time

## Presets (common binding patterns)

```rust
use bevy_enhanced_input::preset::*;

actions!(Player[
    (
        Action::<Movement>::new(),
        DeadZone::default(),
        SmoothNudge::default(),
        DeltaScale::default(),
        Scale::splat(30.0),
        Bindings::spawn((
            Cardinal::wasd_keys(),          // WASD
            Axial::left_stick(),            // gamepad left stick
        )),
    ),
    (
        Action::<Zoom>::new(),
        Bindings::spawn((
            Bidirectional::new(GamepadButton::DPadUp, GamepadButton::DPadDown),
            Spawn((Binding::mouse_wheel(), SwizzleAxis::YXZ)),
        )),
    ),
]);
```

Presets:
- `Cardinal` — four directions (WASD, arrow keys, etc.)
- `Axial` — two-axis analog (left stick, right stick)
- `Bidirectional` — positive/negative pair (DPad, triggers)
- `Ordinal` — diagonal directions
- `Spatial` — 2D spatial input (mouse position, touch)

Connect preset fields to separate binding slots for rebinding:

```rust
Bindings::spawn((
    Cardinal { north: settings.forward, east: settings.right, south: settings.backward, west: settings.left },
));
```

## Multiple contexts

Contexts define when actions are evaluated.

```rust
#[derive(Component)]
struct OnFoot;

#[derive(Component)]
struct InVehicle;

app.add_input_context::<OnFoot>()
   .add_input_context::<InVehicle>();

commands.spawn((
    Player,
    ContextActivity::<InVehicle>::INACTIVE, // start on foot
    actions!(OnFoot[
        (Action::<Jump>::new(), bindings![KeyCode::Space]),
    ]),
    actions!(InVehicle[
        (Action::<Accelerate>::new(), bindings![GamepadButton::RightTrigger2]),
    ]),
));
```

Switch contexts via `ContextActivity`:

```rust
fn enter_vehicle(mut q: Query<&mut ContextActivity<OnFoot>, With<Player>>) {
    let mut activity = q.single_mut();
    *activity = ContextActivity::INACTIVE;
}
```

For games with multiple contexts you can query for specific action or iterate over action contexts.

```rust
fn apply_input(
    jumps: Query<&ActionEvents, With<Action<Jump>>>,
    movements: Query<&Action<Movement>>,
    mut players: Query<(&mut Transform, &Actions<Player>)>,
) {
    for (mut transform, actions) in &mut players {
        let Some(jump_events) = jumps.iter_many(actions).next() else {
            continue;
        };
        let Some(movement) = movements.iter_many(actions).next() else {
            continue;
        };

        // Jumped this frame
        if jump_events.contains(ActionEvents::STARTED) {
            // User logic...
        }

        // We defined the output of `Movement` as `Vec2`,
        // but since translation expects `Vec3`, we extend it to 3 axes.
        transform.translation = movement.extend(0.0);
    }
}
```

## Context priority

```rust
commands.spawn((OnFoot, ContextPriority::<OnFoot>(10))); // higher = evaluated first
```

Default: reverse spawn order (most recently spawned context evaluated first).

## Context activity and state integration

Sync contexts to Bevy states:

```rust
app.sync_context_to_state::<Player, GameState>();

commands.spawn((
    Player,
    ContextActivity::<Player>::INACTIVE,
    ActiveInStates::<Player, _>::single(GameState::Playing),
    actions!(Player[(Action::<Jump>::new(), bindings![KeyCode::Space])]),
));
```

## Action settings

```rust
commands.spawn((
    Action::<Jump>::new(),
    ActionSettings {
        consume_input: true,        // action consumes binding values, preventing lower-priority actions from receiving them
        accumulate: Accumulation::Sum,  // how multiple bindings combine: Sum, Min, Max, Replace
        reset_to_none_each_frame: true, // reset trigger state each frame
    },
    bindings![KeyCode::Space, KeyCode::KeyJ],
));
```

## Mocking actions (testing, AI, cutscenes, networking)

```rust
// Single update:
commands.mock_once::<Jump, Player>();

// With a specific value and duration:
commands.mock::<Movement, Player>(
    ActionMock::new(ActionValue::Vec2(Vec2::X), MockSpan::Ticks(60)),
);

// Remove mock:
commands.unmock::<Movement, Player>();
```

## Gamepad device binding

```rust
commands.spawn((Player, GamepadDevice(Gamepad { id: 0 })));
```

Default: reads from all connected gamepads.

## Action sources (UI blocking)

```rust
fn pause_ui_actions(mut sources: ResMut<ActionSources>) {
    sources.block(Entity::PLACEHOLDER); // block actions while UI is focused
}
```

## Logging / debugging

```sh
RUST_LOG=bevy_enhanced_input=debug cargo run
```

## Removing a context

```rust
player.remove_with_requires::<OnFoot>()
      .despawn_related::<Actions<OnFoot>>();
```
