---
name: bevy-timers
description: Reference for Bevy timers — Timer as resource, component, or Local; tick, finished/just_finished, TimerMode::Once vs Repeat.
metadata:
  crate: bevy_time
  bevy: "0.18"
---

## Timer modes

- `TimerMode::Once` — ticks to 0 once, resets manually
- `TimerMode::Repeat` — auto-resets when finished

Timers tick **up** from zero to their `Duration`.

## Timer as resource

```rust
#[derive(Resource)]
struct MatchTime(Timer);

impl Default for MatchTime {
  fn default() -> Self { Self(Timer::from_seconds(60.0, TimerMode::Once)) }
}

fn countdown(time: Res<Time>, mut match_time: ResMut<MatchTime>) {
  match_time.0.tick(time.delta());
}

fn end_match(match_time: Res<MatchTime>) {
  if match_time.0.finished() { /* game over */ }
}
```

## Timer as component

```rust
#[derive(Component)]
struct Cooldown(Timer);

fn tick_cooldowns(mut commands: Commands, mut cooldowns: Query<(Entity, &mut Cooldown)>, time: Res<Time>) {
  for (entity, mut cd) in &mut cooldowns {
    cd.0.tick(time.delta());
    if cd.0.finished() {
      commands.entity(entity).remove::<Cooldown>();
    }
  }
}
```

## Local timer

```rust
fn local_timer(time: Res<Time>, mut timer: Local<Timer>) {
  timer.tick(time.delta());
  if timer.just_finished() {
    info!("Timer finished");
  }
}
```

## Key methods

| Method | Description |
|--------|-------------|
| `tick(delta)` | Advance the timer |
| `finished()` | True if timer has reached duration |
| `just_finished()` | True if finished on the last tick |
| `fraction()` | Progress as `f32` (0.0 to 1.0) |
| `reset()` | Reset to zero |
| `duration()` / `set_duration()` | Get/set duration |

## Time resource

```rust
fn time_info(time: Res<Time>) {
  info!("delta: {:?}", time.delta());
  info!("elapsed: {:?}", time.elapsed());
}
```
