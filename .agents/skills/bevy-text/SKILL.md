---
name: bevy-text
description: Reference for text rendering in Bevy — Text vs Text2d, styling, text spans, changing text at runtime, clicking text, and font rendering.
metadata:
  crate: bevy_text
  bevy: "0.18"
---

## Text vs Text2d

| Component | Use case |
|-----------|----------|
| `Text` | UI text (fixed position, in HUD/inventory) |
| `Text2d` | Scene text (floating damage, world-space labels) |

## Creating text

```rust
// In scene
commands.spawn((
  Text2d::new("Hello"),
  TextColor(Color::WHITE),
  TextFont::from(font_handle).with_font_size(60.),
  TextLayout::new_with_justify(Justify::Center),
));

// In UI
commands.spawn((
  Node { position_type: PositionType::Absolute, bottom: px(5.0), right: px(5.0), ..default() },
  Text::new("Hello"),
  TextFont::from(font_handle),
));
```

## Styling components

- `TextLayout` — alignment, line break behavior
- `TextFont` — font, font size
- `TextColor` — color override

## Multi-style text (TextSpan)

```rust
commands.spawn((
  Text::new("Here is some text. But "),
  TextFont::from(regular_font),
  children![(
    TextSpan::new("this part is bold"),
    TextFont::from(bold_font),
  )],
));
```

## Changing text at runtime

```rust
fn update(mut texts: Query<&mut Text, With<ItemName>>) {
  for mut text in &mut texts {
    text.0 = "New text".to_string();
  }
}
```

## Clicking text

```rust
fn handle_click(event: On<Pointer<Click>>, mut query: Query<&mut Text2d>) {
  query.get_mut(event.entity).unwrap().0 = "Clicked!".to_string();
}

commands.spawn(Text2d::new("Click me")).observe(handle_click);
```

## Font files

Supported: `ttf`, `otf`. Default font: Fira Mono. Load via `AssetServer`.

Bevy 0.18+ supports OpenType font features.
