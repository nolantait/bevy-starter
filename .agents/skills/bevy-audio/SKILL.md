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
| `PlaybackSettings::REMOVE` | Play once, then remove the audio components from the entity |

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
| `is_paused` / `is_muted` | Playback state |
| `speed` / `set_speed` | Playback speed |
| `position` | Current playback position |
| `empty` | True if no more sounds to play |
| `try_seek` | Seek to a position |
| `volume` / `set_volume` | Volume control |

These methods come from the `AudioSinkPlayback` trait, implemented by both
`AudioSink` and `SpatialAudioSink`.

## Volume

Global volume via `GlobalVolume` resource:

```rust
app.insert_resource(GlobalVolume::new(Volume::Linear(0.5)));
```

Per-sink volume:

```rust
sink.set_volume(current_volume.increase_by_percentage(10.0));
```

Changing `GlobalVolume` does **not** affect audio that is already playing; the
new value only applies when a sink is created. The same is true for changing
`PlaybackSettings` on a playing entity.

## Spatial audio

By default sounds are flat. Set `spatial: true` on `PlaybackSettings` (and give
the entity a `Transform`) to make a sound spatial, then use a `SpatialListener`
component to control how it is heard. Only one `SpatialListener` should exist at
a time.

```rust
app.add_plugins(DefaultPlugins.set(AudioPlugin {
  default_spatial_scale: SpatialScale::new_2d(1.0 / 100.0),
  ..default()
}));

commands.spawn((SpatialListener::new(100.), Transform::default()));
```

## Supported formats

`wav`, `ogg` (plus `oga` and `spx`), `flac`, `mp3` (enable the `mp3` feature).

## Simple tone (Pitch)

```rust
commands.spawn((
  AudioPlayer(pitch_assets.add(Pitch::new(220.0, Duration::new(1, 0)))),
  PlaybackSettings::DESPAWN,
));
```
