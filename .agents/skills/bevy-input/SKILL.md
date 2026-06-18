---
name: bevy-input
description: Reference for handling input in Bevy — keyboard, mouse, touch, gamepad, events vs resources, physical vs logical keys, and enhanced input.
metadata:
  crate: bevy_input
  bevy: "0.18"
---

## Two approaches

1. **Events** — `EventReader<T>` for reacting to any input of a type
2. **Resources** — `ButtonInput<T>`, `Axis<T>`, `Touches`, `Gamepads` for specific state queries

## Input resources

| Resource | Purpose |
|----------|---------|
| `ButtonInput<KeyCode>` | Keyboard keys |
| `ButtonInput<MouseButton>` | Mouse buttons |
| `Axis<GamepadAxis>` | Gamepad analog sticks/triggers |
| `ButtonInput<GamepadButton>` | Gamepad buttons |
| `Touches` | Touch state |
| `Gamepads` | Connected gamepads |

## ButtonInput methods

| Method | Returns true |
|--------|-------------|
| `pressed(k)` | Between press and release |
| `just_pressed(k)` | One frame after press |
| `just_released(k)` | One frame after release |
| `any_pressed([k1, k2])` | Any in list pressed |

## Keyboard

```rust
fn jump(input: Res<ButtonInput<KeyCode>>) {
  if input.just_pressed(KeyCode::Space) { /* jump */ }
}
```

### Physical vs logical keys

- `key_code` — physical position on keyboard (use for gameplay)
- `logical_key` — mapped to OS layout (use for text)

## Mouse

```rust
fn shoot(mouse: Res<ButtonInput<MouseButton>>) {
  if mouse.just_pressed(MouseButton::Left) { }
}
```

Mouse motion, cursor, wheel, gestures — read via `EventReader`:

```rust
fn mouse_events(
  mut motion: EventReader<MouseMotion>,
  mut cursor: EventReader<CursorMoved>,
  mut wheel: EventReader<MouseWheel>,
) {
  for ev in motion.read() { }
  for ev in cursor.read() { }
  for ev in wheel.read() { }
}
```

## Touch

```rust
fn touch(touches: Res<Touches>) {
  for touch in touches.iter_just_pressed() { }
  for touch in touches.iter_just_released() { }
  for touch in touches.iter() { }
}
```

## Gamepad

```rust
fn gamepad(gamepads: Query<&Gamepad>, buttons: Res<ButtonInput<GamepadButton>>) {
  for pad in &gamepads {
    if buttons.just_pressed(GamepadButton::South) { }
  }
}
```

### Haptics

```rust
commands.queue(GamepadRumbleRequest::Add {
  gamepad: entity,
  intensity: GamepadRumbleIntensity::strong_motor(0.5),
  duration: Duration::from_millis(300),
});
```

## Modifiers (Shift/Ctrl)

```rust
let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
if ctrl && shift && input.just_pressed(KeyCode::KeyA) { }
```

## Enhanced input (observer pattern)

Use observers to avoid scheduling issues with `FixedUpdate`:

```rust
fn apply_movement(trigger: Trigger<Fired<Move>>, mut players: Query<&mut Transform>) {
  let mut t = players.get_mut(trigger.target()).unwrap();
  t.translation += trigger.value.extend(0.0);
}
```
