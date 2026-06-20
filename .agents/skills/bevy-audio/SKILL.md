---
name: bevy-audio
description: Reference for playing and controlling audio in Bevy — AudioPlayer, AudioSink, spatial audio, volume, and playback settings.
metadata:
  crate: bevy_audio
  bevy: "0.19"
---

## Core components

- `AudioPlayer<Source>` — component to play an audio source (requires `PlaybackSettings`)
- `AudioSink` — component added automatically; controls playback on that entity
- `Pitch` — asset for generating simple tones

## Playing audio

```rust
fn play(server: Res<AssetServer>, mut commands: Commands) {
  commands.spawn((
    AudioPlayer::new(server.load("music.ogg")),
    PlaybackSettings::LOOP,
  ));
}
```

## Playback settings

| Setting | Behavior |
|---------|----------|
| `PlaybackSettings::ONCE` | Play once |
| `PlaybackSettings::LOOP` | Loop continuously |
| `PlaybackSettings::DESPAWN` | Play once, then despawn entity |
| `PlaybackSettings::REMOVE` | Play once, then remove component |

## Controlling playback (AudioSink)

```rust
fn toggle(mut sink: Single<&mut AudioSink, With<MusicBox>>) {
  sink.toggle_playback();
}
```

| Method | Description |
|--------|-------------|
| `play` / `pause` / `stop` | Basic playback control. `stop` cannot restart. |
| `mute` / `unmute` / `toggle_mute` | Mute control |
| `toggle_playback` | Toggle play/pause |
| `speed` / `set_speed` | Playback speed |
| `empty` | True if no more sounds to play |
| `try_seek` | Seek to a position |
| `volume` / `set_volume` | Volume control |

## Volume

Global volume via `GlobalVolume` resource:

```rust
app.insert_resource(GlobalVolume::new(Volume::Linear(0.5)));
```

Per-sink volume:

```rust
sink.set_volume(current_volume.increase_by_percentage(10.0));
```

## Spatial audio

```rust
app.add_plugins(DefaultPlugins.set(AudioPlugin {
  default_spatial_scale: SpatialScale::new_2d(1.0 / 100.0),
  ..default()
}));
```

Spawn a `SpatialListener` (only one) for positional audio:

```rust
commands.spawn((SpatialListener::new(100.), Transform::default()));
```

## Supported formats

`ogg` (default), `wav`, `flac`, `mp3` (enable `mp3` feature).

## Simple tone (Pitch)

```rust
commands.spawn((
  AudioPlayer(pitch_assets.add(Pitch::new(220.0, Duration::new(1, 0)))),
  PlaybackSettings::DESPAWN,
));
```
