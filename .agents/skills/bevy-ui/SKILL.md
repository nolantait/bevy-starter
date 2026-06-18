---
name: bevy-ui
description: Reference for Bevy UI — Node layout (Flexbox/CSS Grid), Val types, text, colors, buttons, interaction, scrolling, and relative cursor position.
metadata:
  crate: bevy_ui
  bevy: "0.18"
---

## Core concept

UI is built with entities + components. `Node` controls layout via Flexbox or CSS Grid (powered by [taffy](https://github.com/DioxusLabs/taffy)).

UI is rendered independently of camera viewport (stays put when camera moves). Use `UiTargetCamera` to follow a camera.

## Node layout

```rust
commands.spawn((
  Node {
    width: Val::Percent(100.),
    height: Val::Percent(100.),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
  },
  BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
));
```

## Val types

| Type | Description |
|------|-------------|
| `Val::Auto` | Automatic |
| `Val::Px(f32)` | Pixel value |
| `Val::Percent(f32)` | Percentage of parent |
| `Val::Vw(f32)` / `Val::Vh(f32)` | Viewport width/height % |
| `Val::VMin(f32)` / `Val::VMax(f32)` | Viewport min/max % |

Helper functions: `px(10.)`, `percent(50.)`, `vw(20.)`, etc.

## Colors

```rust
// From CSS palette
use bevy::color::palettes::css::{BLACK, BLUE, WHITE};

// SRGB
let color = Color::srgb(0.0, 0.0, 0.0);
let color: Hsla = Srgba::rgb(1.0, 0.0, 1.0).into();
// Alpha: .set_alpha(), .with_alpha()
```

## Text in UI

```rust
commands.spawn((
  Node { position_type: PositionType::Absolute, bottom: px(5.), right: px(5.), ..default() },
  Text::new("Hello"),
  TextColor(Color::BLACK),
  TextLayout::new_with_justify(Justify::Center),
));
```

## Buttons and interaction

Using picking observers:

```rust
fn on_hover(event: On<Pointer<Over>>, mut commands: Commands) {
  commands.entity(event.entity).insert(BackgroundColor(GREEN.into()));
}
```

Or querying `Interaction` component directly:

```rust
fn button_system(mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>) {
  for (interaction, mut color) in &mut query {
    match interaction {
      Interaction::Pressed => *color = PRESSED_BUTTON.into(),
      Interaction::Hovered => *color = HOVERED_BUTTON.into(),
      Interaction::None => *color = NORMAL_BUTTON.into(),
    }
  }
}
```

## Children

```rust
commands.spawn((container, children![(child_node, child_text)]));
```

## Z-ordering

First node in `UiStack` is furthest, rendered first. Last node receives interactions first.

- `ZIndex` — local z-order within parent
- `GlobalZIndex` — global z-order across all nodes

## Relative cursor position

```rust
use bevy::ui::RelativeCursorPosition;

// Spawn on a node to auto-track:
commands.spawn((Node { .. }, RelativeCursorPosition::default()));

// Query:
fn cursor_pos(query: Query<&RelativeCursorPosition>) {
  if let Ok(cursor) = query.single() {
    if let Some(pos) = cursor.normalized {
      info!("({:.1}, {:.1})", pos.x, pos.y);
    }
  }
}
```

## Scrolling

```rust
Node {
  overflow: Overflow::scroll_y(),
  ..default()
}
```

## Spawning UI elements structure

```rust
fn button() -> impl Bundle {
  (
    Button,
    Node { width: px(150), height: px(65), border: UiRect::all(px(5)),
           justify_content: JustifyContent::Center,
           align_items: AlignItems::Center, ..default() },
    BorderColor::all(Color::WHITE),
    BackgroundColor(Color::BLACK),
    children![(Text::new("Button"), TextColor(Color::srgb(0.9, 0.9, 0.9)))],
  )
}
```
