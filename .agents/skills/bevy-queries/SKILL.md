---
name: bevy-queries
description: Reference for writing Bevy ECS queries — Query<D, F>, Single, filters, component access, iteration patterns, lenses, disjointed access, and testing.
metadata:
  crate: bevy_ecs
  bevy: "0.19"
---

## Basics

`Query<D, F>` is a system parameter. `D` = query data (what to fetch), `F` = query filter (conditions). Data is only fetched when iterated.

```rust
fn system(query: Query<&Transform>) {
  for transform in &query { }
}
```

## Component mutability

| Syntax | Meaning |
|--------|---------|
| `Query<&T>` | Readonly borrow — parallel-friendly |
| `Query<&mut T>` | Mutable borrow — blocks parallel access to same component |
| `Query<Option<&T>>` | Optional component — entity may or may not have it |

## Tuple = AND logic

Each generic parameter in `Query<D, F>` can be a tuple. All types must match.

```rust
Query<(&Ball, &Player)>                // entities with both Ball AND Player
Query<&Transform, (With<Player>, With<Living>)>  // Transform on entities with Player AND Living
```

## Alternative query parameters

| Parameter | Behavior |
|-----------|----------|
| `Single<D, F>` | Exactly one match, or system skipped |
| `Option<Single<D, F>>` | Zero or one match; yields `None` if there are none or more than one |
| `Populated<D, F>` | One or more matches, skips if none |

```rust
fn move(mut t: Single<&mut Transform, With<Player>>) {
  t.translation.x += 1.;
}
fn destructure(s: Single<(&mut Pos, &Vel), With<Player>>) {
  let (mut pos, vel) = s.into_inner();
}
```

## QueryData (D parameter)

| Type | Description |
|------|-------------|
| `&T` / `&mut T` | Read or write component |
| `Option<T>` | Component or `None` |
| `AnyOf<T>` | Fetch entities matching any of the tuple types |
| `Ref<T>` | Readonly with change detection methods |
| `Has<T>` | Returns `bool` if entity has component |
| `Entity` | The entity ID |
| `SpawnDetails` | When paired with the `Spawned` filter, gives access to when and where an entity was spawned |

### AnyOf

```rust
Query<AnyOf<(&Player, &Rocket, &mut Astroid)>>
// Expands to: Query<(Option<&P>, Option<&R>, Option<&mut A>), Or<(With<P>, With<R>, With<A>)>>
```

### Ref (change detection)

```rust
fn check(q: Query<Ref<Player>>) {
  for p in &q {
    if p.is_added() { }
    if p.is_changed() { }
    // p.last_changed()
  }
}
```

### Entity ID

```rust
fn with_id(q: Query<(Entity, &Transform)>) {
  for (entity, transform) in &q { }
}
fn lookup(players: Query<Entity, With<Player>>, transforms: Query<&Transform>) {
  for e in &players {
    let t = transforms.get(e).unwrap();
  }
}
```

## QueryFilter (F parameter)

| Filter | Description |
|--------|-------------|
| `With<T>` | Only entities with component T |
| `Without<T>` | Only entities without component T |
| `Or<F>` | Checks if any filters in the tuple `F` apply |
| `Changed<T>` | Components of type T that changed since the system last ran |
| `Added<T>` | Components of type T that were added since the system last ran |
| `Spawned` | Only entities that were spawned since the system last ran |

```rust
Query<&Transform, (With<Player>, Without<Dead>)>
Query<Player, Added<Player>>  // equivalent to Ref<Player> + is_added check
Query<(&Player, SpawnDetails), Spawned>  // newly spawned entities
```

## Retrieval methods

| Method | Description |
|--------|-------------|
| `iter` / `iter_mut` | Iterator over all matches |
| `iter_many` / `iter_many_mut` | Iterate over only the items matching a list of entities |
| `iter_combinations` / `iter_combinations_mut` | All K-combinations of matches |
| `contiguous_iter` / `contiguous_iter_mut` | Receives whole table slices at once, enabling SIMD optimizations |
| `par_iter` / `par_iter_mut` | Parallel iterator |
| `get` / `get_mut` | Fetch a single entity's components by `Entity` |
| `get_many` / `get_many_mut` | Fetch items for each entity in a fixed-size array |
| `single` / `single_mut` | The only query item as a `Result`, `Err` unless there is exactly one |
| `is_empty` | Check if query has matches |
| `contains` | Check if query contains a specific entity |

Every method that returns query items has a `*_mut` variant; the `*_mut` methods
require a mutable `Query` parameter.

```rust
// Iteration
for mut t in &mut query { }
query.iter_mut().for_each(|mut t| { });

// Specific entity
if let Ok(t) = query.get(entity) { }

// Combinations
for [a, b] in query.iter_combinations() { }

// Many entities
let mut iter = query.iter_many_mut(&entities);
while let Some(mut h) = iter.fetch_next() { }
```

## Query lenses

Share common query logic without duplicating system parameters.

```rust
fn print_health(lens: &mut QueryLens<&Health>) {
  for h in &mut lens.query() {
    if h.0 > 50.0 { info!("healthy"); }
  }
}
fn player_system(mut q: Query<(&Health, &Player)>) {
  print_health(&mut q.transmute_lens::<&Health>());
}
fn enemy_system(mut q: Query<(&Health, &Enemy, &Transform)>) {
  print_health(&mut q.transmute_lens::<&Health>());
}
```

- `transmute_lens` — narrow query data
- `transmute_lens_filtered` — include filter
- `join` / `join_filtered` — combine queries

## Disjointed queries & ParamSet

Two queries with mutable access to overlapping component sets: use `Without` to disambiguate, or use `ParamSet` (which serializes access at runtime).

```rust
// Will panic at runtime — ambiguous borrows
fn bad(p: Query<&mut Player, With<Rocket>>, e: Query<&mut Player, With<Invincibility>>) { }

// Safe: serialized access
fn ok(p: ParamSet<(Query<&mut Player, With<Rocket>>, Query<&mut Player, With<Invincibility>>)>) { }

// Or with disjoint archetypes (Bevy 0.12+):
fn disjoint(t: Query<EntityMut, With<Transform>>, e: Query<EntityMut, Without<Transform>>) { }
```

## Performance notes

- `Table` storage iterates faster than `SparseSet`
- Two systems with conflicting mutable access to the same component type cannot run in parallel
- `for_each` is generally faster than `iter` on worlds with high archetype fragmentation
- Prefer `iter` over `for_each` unless profiling shows a need
- Accessing `entity.get_components_mut::<(&mut A, &mut B)>()` has quadratic cost over number of components

## Testing

Use `app.world_mut().run_system_once(fn)` or access `app.world_mut().query::<T>()`:

```rust
#[cfg(test)]
mod tests {
  use super::*;

  fn setup_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, plugin));
    app
  }

  fn check_ship(query: Query<&Ship>) {
    assert_eq!(query.iter().count(), 1);
  }

  #[test]
  fn test_spawn() {
    let mut app = setup_app();
    app.update();
    app.world_mut().run_system_once(check_ship).unwrap();
  }
}
```
