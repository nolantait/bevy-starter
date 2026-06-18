---
name: bevy-picking
description: Reference for Bevy's picking system — pointer events, backends, hover/click/drag observers, sprite picking, UI picking, and mesh picking.
metadata:
  crate: bevy_picking
  bevy: "0.18"
---

## Core concept

`Pointer` is an abstract representation of user input at a screen location. **Backends** read `PointerLocation` components and produce `PointerHits` events. The picking pipeline handles the rest.

## Default picking plugins

`DefaultPickingPlugins` (included with `DefaultPlugins` when `bevy_picking` feature enabled) contains:

- `InteractionPlugin` — generates pointer events, handles bubbling
- `PickingPlugin` — manages picking state, produces higher-level events
- `PointerInputPlugin` — mouse and touch events

## Pointer event types

### Hovering
- `Pointer<Over>` — pointer entered entity bounds
- `Pointer<Move>` — pointer moving over entity
- `Pointer<Out>` — pointer left entity bounds

### Clicking
- `Pointer<Press>` / `Pointer<Release>` — button pressed/released
- `Pointer<Click>` — press + release on same entity

### Dragging
- `Pointer<DragStart>` / `Pointer<Drag>` / `Pointer<DragEnd>`
- `Pointer<DragEnter>` / `Pointer<DragOver>` / `Pointer<DragDrop>` / `Pointer<DragLeave>`

## Picking sprites

Enable with `Pickable` component:

```rust
commands.spawn((
  Sprite::from_color(GREEN, Vec2::new(100., 100.)),
  Pickable::default(),
)).observe(|hover: On<Pointer<Over>>, mut sprites: Query<&mut Sprite>| {
  sprites.get_mut(hover.entity).unwrap().color = YELLOW.into();
});
```

## Picking UI

UI nodes are pickable by default. Attach observers to specific elements:

```rust
commands.spawn(button()).observe(|click: On<Pointer<Click>>| {
  info!("Button clicked!");
});
```

## Picking meshes (3D)

Add `MeshPickingPlugin`. Use observers on mesh entities:

```rust
commands.spawn((
  Mesh3d(meshes.add(Cuboid::from_length(5.))),
  MeshMaterial3d(materials.add(Color::from(SILVER))),
)).observe(|drag: On<Pointer<Drag>>, mut transforms: Query<&mut Transform>| {
  let mut t = transforms.get_mut(drag.entity).unwrap();
  t.rotate_y(drag.delta.x * 0.02);
  t.rotate_x(drag.delta.y * 0.02);
});
```

## Ignoring entities

```rust
Pickable::IGNORE  // disables picking on this entity
```

Set `MeshPickingSettings::require_markers` to true for opt-in mesh picking.

## Picking pipeline order

1. Input → update pointers → `PointerInput` events
2. Update `PointerLocation` components
3. Backends read locations → produce `PointerHits`
4. Build `HoverMap` (topmost entity wins)
5. Generate higher-level `Pointer<E>` events

Within a frame: `Out`/`DragLeave` first, then `DragEnter`/`Over`, then movement/press/drag events in any order.
