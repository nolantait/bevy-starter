---
name: bevy-input
description: Reference for handling input in Bevy — keyboard, mouse, touch, gamepad, events vs resources, physical vs logical keys, and enhanced input.
metadata:
  crate: bevy_input
  bevy: "0.19"
---

## Two approaches

1. **Messages** — read `MessageReader<T>` for input emitted by Bevy's input systems
2. **Polling** — resources like `ButtonInput<T>` / `Touches`, or querying components like `Gamepad`

## Input types

| Type | Description |
|------|-------------|
| `ButtonInput<T>` | A "press-able" input, such as `KeyCode` or `MouseButton` |
| `Axis<T>` | Position data from certain input devices |
| `Touches` | A collection of `Touch`es that have happened |
| `TouchInput` | Messages representing touch-based input |
| `Gamepad` | Component on each connected controller, holding its button/axis state |
| `GamepadAxis` | Enum describing the axes available on a gamepad |

## ButtonInput methods

| Method | Returns true |
|--------|-------------|
| `pressed(k)` | Between press and release |
| `just_pressed(k)` | On the frame the press was processed |
| `just_released(k)` | On the frame the release was processed |
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

Mouse motion, cursor, wheel, gestures — read via `MessageReader`:

```rust
fn mouse_events(
  mut motion: MessageReader<MouseMotion>,
  mut cursor: MessageReader<CursorMoved>,
  mut wheel: MessageReader<MouseWheel>,
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

`Gamepad` is a component on each connected controller. Poll its own state, or
use the `ButtonInput<GamepadButton>` / `Axis<GamepadAxis>` resources:

```rust
fn gamepad(gamepads: Query<&Gamepad>, buttons: Res<ButtonInput<GamepadButton>>) {
  for pad in &gamepads {
    if pad.just_pressed(GamepadButton::South) { }
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
fn apply_movement(trigger: On<Fire<Move>>, mut players: Query<&mut Transform>) {
  let mut t = players.get_mut(trigger.context).unwrap();
  t.translation += trigger.value.extend(0.0);
}
```
