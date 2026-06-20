---
name: bevy-relationships
description: Reference for Bevy entity relationships — ChildOf/Children hierarchy, custom relationships with Relationship/RelationshipTarget, spawning children, and traversal.
metadata:
  crate: bevy_ecs
  bevy: "0.19"
---

## Built-in parent/child

Two components kept in sync:

- `ChildOf(Entity)` — the relationship, added to child entities
- `Children(Vec<Entity>)` — the target, auto-maintained on parent

Transform propagation: parent's `Transform`/`GlobalTransform` auto-propagates to children. Despawning parent removes all `ChildOf` components.

### Spawning children

```rust
// Via with_children
commands.spawn(Fleet).with_children(|parent| {
  parent.spawn((Ship, Name::new("Ship 1")));
});

// Via children! macro
commands.spawn((Fleet, children![(Ship, Name::new("Ship 2"))]));
```

## Custom relationships

Define a `Relationship` and a `RelationshipTarget`:

```rust
#[derive(Component)]
#[relationship(relationship_target = ShipAttachments)]
struct AttachedToShip(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = AttachedToShip, linked_spawn)]
struct ShipAttachments(Vec<Entity>);
```

`linked_spawn` auto-despawning children when parent is removed.

### Spawning with custom relationship

```rust
let ship = commands.spawn(Ship).id();
commands.spawn((GunTurret, AttachedToShip(ship)));

// Or with related! macro:
commands.spawn((Ship, related!(ShipAttachments[
  (GunTurret, Name::new("Turret 1")),
])));
```

## Querying relationships

```rust
// Parent → children
fn from_ship(ships: Query<&ShipAttachments>, turrets: Query<&Name, With<GunTurret>>) {
  for attachments in &ships {
    for &attachment in &attachments.0 {
      if let Ok(name) = turrets.get(attachment) { }
    }
  }
}

// Children → parent (iter_ancestors)
fn from_turret(turrets: Query<Entity, With<AttachedToShip>>, att: Query<&AttachedToShip>) {
  for turret in &turrets {
    for parent in att.iter_ancestors(turret) { }
  }
}

// Parent → children (iter_descendants)
fn from_ship_desc(ships: Query<Entity, With<Ship>>, att: Query<&ShipAttachments>) {
  for ship in &ships {
    for child in att.iter_descendants(ship) { }
  }
}
```

## Limitations

- No native many-to-many relationships
- `ChildOf` can only point to a single entity
- Relationships don't fragment archetypes like other components
