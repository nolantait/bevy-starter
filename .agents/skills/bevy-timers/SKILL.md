---
name: bevy-timers
description: Reference for Bevy timers — Timer as resource, component, or Local; tick, is_finished/just_finished, TimerMode::Once vs Repeating.
metadata:
  crate: bevy_time
  bevy: "0.19"
---

## Timer modes

- `TimerMode::Once` — stays finished after reaching the end until reset manually
- `TimerMode::Repeating` — auto-resets when it reaches the end

Timers tick **up** from zero to their `Duration`.

`is_finished` stays `true` after the tick that finished a `Once` timer, while
`just_finished` is only `true` on that tick. For a `Repeating` timer the two are
equivalent. A repeating timer can finish more than once per tick, so use
`times_finished_this_tick` to know how many times it finished.

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
  if match_time.0.is_finished() { /* game over */ }
}
```

Implement `Default` by hand for this kind of wrapper: a derived `Default` would
create a zero-length timer, since that is what `Timer::default` returns.

## Timer as component

```rust
#[derive(Component)]
struct Cooldown(Timer);

fn tick_cooldowns(mut commands: Commands, mut cooldowns: Query<(Entity, &mut Cooldown)>, time: Res<Time>) {
  for (entity, mut cd) in &mut cooldowns {
    cd.0.tick(time.delta());
    if cd.0.is_finished() {
      commands.entity(entity).remove::<Cooldown>();
    }
  }
}
```

## Local timer

`Local` values are initialized from `Default`, and a default `Timer` has a
zero-length duration, so wrap it in a type that builds a real timer through
`FromWorld`:

```rust
struct LocalTimer(Timer);

impl FromWorld for LocalTimer {
  fn from_world(_world: &mut World) -> Self {
    Self(Timer::from_seconds(5.0, TimerMode::Once))
  }
}

fn local_timer(time: Res<Time>, mut timer: Local<LocalTimer>) {
  timer.0.tick(time.delta());
  if timer.0.just_finished() {
    info!("Timer finished");
  }
}
```

## Key methods

| Method | Description |
|--------|-------------|
| `tick(delta)` | Advance the timer |
| `is_finished()` | True if timer has reached duration |
| `just_finished()` | True if finished on the last tick |
| `times_finished_this_tick()` | How often a repeating timer finished this tick |
| `fraction()` / `fraction_remaining()` | Progress / remaining as `f32` (0.0 to 1.0) |
| `pause()` / `unpause()` | Pause or resume |
| `reset()` / `finish()` | Reset to zero / snap to finished |
| `duration()` / `set_duration()` | Get/set duration |
| `elapsed()` / `remaining()` | Elapsed or remaining time |

Prefer the run conditions `on_timer`, `on_real_timer`, `once_after_delay`, and
`repeating_after_delay` to run a system on an interval.

## Time resource

`Res<Time>` is `Time<Virtual>` (affected by pause/speed). Use `Time<Real>` for
unscaled time and `Time<Fixed>` inside `FixedUpdate`.

```rust
fn time_info(time: Res<Time>) {
  info!("delta: {:?}", time.delta());
  info!("seconds: {:?}", time.delta_secs_f64());
  info!("elapsed: {:?}", time.elapsed());
}
```
