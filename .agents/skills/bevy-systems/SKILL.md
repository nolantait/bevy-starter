---
name: bevy-systems
description: Reference for Bevy systems — scheduling, ordering, system parameters, ParamSet, exclusive systems, fallibility, custom system params, and run conditions.
metadata:
  crate: bevy_ecs
  bevy: "0.19"
---

## System basics

A system is a Rust function whose parameters all implement `SystemParam`. Bevy auto-converts via `IntoSystem`.

```rust
fn hello() { println!("Hello!"); }
```

## Scheduling

```rust
app.add_systems(Update, hello);
app.add_systems(Update, (defend, attack));
```

### Schedule labels

Main labels:

1. `Update` — runs once every loop
2. `FixedUpdate` — runs once every fixed amount of time
3. `Startup` — runs once at startup

Other built-in labels, in order:

1. `PreStartup`
2. `Startup`
3. `PostStartup`
4. `First`
5. `PreUpdate`
6. `StateTransition`
7. `RunFixedMainLoop` — runs the `FixedMain` schedules including `FixedUpdate`
8. `Update`
9. `SpawnScene`
10. `PostUpdate`
11. `Last`

Each label identifies a `Schedule`, which holds the metadata and executor needed
to run its systems. Bevy tries to run systems in parallel unless they have
conflicting mutable data access; archetypes optimize this.

`Commands` queue into a `CommandQueue` and are applied at a **sync point**: an
`ApplyDeferred` system that Bevy inserts after command-producing systems that
others depend on, as well as at the end of every schedule.

## Ordering

```rust
// before/after
app.add_systems(Update, (defend.before(end_turn), attack.after(defend), end_turn));

// chain (sequential)
app.add_systems(Update, (defend, attack, end_turn).chain());
```

## Custom system sets

```rust
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct PhysicsSet;

app.add_systems(Update, (move_objects, collide).in_set(PhysicsSet));
app.configure_sets(Update, (PhysicsSet, EconomySet).chain());
```

## Common system parameters

| Param | Description |
|-------|-------------|
| `Res<T>` / `ResMut<T>` | Read/write resource |
| `Query<D, F>` | Query components |
| `Commands` | Queue world mutations |
| `Local<T>` | Per-system persistent state |
| `ParamSet<(P0, P1)>` | Mutually-exclusive params |
| `&World` | Exclusive world access |
| `MessageReader<T>` / `MessageWriter<T>` | Read/write messages |
| `NonSend<T>` / `NonSendMut<T>` | Non-Send resource |

## ParamSet

For two queries that would conflict (same mutable component):

```rust
fn good(mut set: ParamSet<(Query<&mut Health, With<Enemy>>, Query<&mut Health, With<Ally>>)>) {
  for mut h in set.p0().iter_mut() { }
  for mut h in set.p1().iter_mut() { }  // p0 is no longer borrowed
}
```

## Fallibility

Return `Result` to make a system failable:

```rust
fn failable() -> Result<()> { Ok(()) }
```

Set global error handler:

```rust
GLOBAL_ERROR_HANDLER.set(warn).unwrap();
```

Built-in handlers: `panic`, `error`, `warn`, `info`, `debug`, `trace`, `ignore`.

### Fallible system params (skip system on validation failure)

| Param | Skipped when |
|-------|-------------|
| `Single<D, F>` | Not exactly one match |
| `Option<Single<D, F>>` | More than one match |
| `Populated<D, F>` | No matches |
| `Res<T>` / `ResMut<T>` | Resource doesn't exist (calls error handler) |

## Custom system parameters

```rust
#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
  players: Query<'w, 's, &'static Player>,
  count: ResMut<'w, PlayerCount>,
}
impl PlayerCounter<'_, '_> {
  fn count(&mut self) { self.count.0 = self.players.iter().len(); }
}
```

## System state (Local)

```rust
fn count_calls(mut counter: Local<u32>) {
  *counter += 1;
  println!("Called {} times", *counter);
}
```

## Exclusive systems

Use `&mut World` for immediate (non-deferred) access:

```rust
fn exclusive(world: &mut World) {
  world.spawn(Player);
}
```

Cannot run in parallel with other systems that need `&mut World`.

## Piping systems

```rust
fn parse(input: In<String>) -> usize { input.len() }
fn show(len: In<usize>) { info!("length: {}", len); }
app.add_systems(Update, parse.pipe(show));
```

## Removing systems

```rust
schedule.remove_systems_in_set(MySystem, ScheduleCleanupPolicy::RemoveSystemsOnly);
app.remove_systems_in_set(MySet, ScheduleCleanupPolicy::RemoveSetAndSystems);
```
