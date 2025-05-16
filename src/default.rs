use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

// Sets up the default plugins like windows, assets, etc

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".into(),
                resizable: false,
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }));
}
