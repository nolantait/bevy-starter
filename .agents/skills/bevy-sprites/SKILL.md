---
name: bevy-sprites
description: Reference for Bevy sprites — rendering images, custom size, anchors, z-ordering, sprite sheets, texture atlases, pixel-perfect rendering, and bounding boxes.
metadata:
  crate: bevy_sprite
  bevy: "0.18"
---

## Basic sprite

```rust
commands.spawn(Sprite {
  image: asset_server.load("sprites/ball.png"),
  ..default()
});
```

Shorthand: `Sprite::from_image(handle)` or `Sprite::from_color(color, size)` for prototyping.

## Custom size vs transform scale

- `custom_size: Some(Vec2::new(100., 100.))` — sprite **is** that size
- `Transform::scale` — sprite **looks** scaled (affects children too)

## Anchors

Default is `Anchor::Center`. Other variants: `BOTTOM_LEFT`, `TOP_RIGHT`, etc. Custom: `Anchor(Vec2)` with relative values (e.g., `(-0.5, 0.5)` = top-left).

## Z-ordering

Higher `Transform.translation.z` renders on top:

```rust
commands.spawn((Sprite { .. }, Transform::from_xyz(0., 0., 1.))); // on top
```

## Sprite sheets (TextureAtlasLayout)

```rust
#[derive(Resource)]
struct PlayerSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for PlayerSpriteSheet {
  fn from_world(world: &mut World) -> Self {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let mut atlases = world.get_resource_mut::<Assets<TextureAtlasLayout>>().unwrap();
    Self(atlases.add(layout))
  }
}

fn spawn(mut commands: Commands, sheet: Res<PlayerSpriteSheet>, server: Res<AssetServer>) {
  commands.spawn(Sprite {
    image: server.load("player.png"),
    texture_atlas: Some(TextureAtlas { layout: sheet.0.clone(), index: 0 }),
    ..default()
  });
}
```

## Texture atlas builder

Combine separate images into one sprite sheet:

```rust
let mut builder = TextureAtlasBuilder::default();
builder.add_texture(Some(id), texture);
let (layout, sources, atlas_image) = builder.build().unwrap();
textures.add(atlas_image);
```

## Tiled sprites

```rust
Sprite {
  image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: 0.5 },
  ..default()
}
```

## Bounding box of transformed sprites

```rust
fn bounds(sprites: Query<(&Transform, &Sprite)>, assets: Res<Assets<Image>>) {
  for (t, sprite) in &sprites {
    let image_size = assets.get(&sprite.image).unwrap().size_f32();
    let scaled = image_size * t.scale.truncate();
    let bbox = Rect::from_center_size(t.translation.truncate(), scaled);
  }
}
```

## Pixel-perfect rendering

```rust
app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
```
