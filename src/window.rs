use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

pub(super) fn plugin(app: &mut App) {
    let primary_window = Window {
        title: "Bevy game".into(),
        resizable: false,
        resolution: (800., 600.).into(),
        canvas: Some("#bevy".to_owned()),
        desired_maximum_frame_latency: core::num::NonZero::new(1u32),
        ..default()
    };

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(primary_window),
            ..default()
        }));
}
