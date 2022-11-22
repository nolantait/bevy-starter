use bevy::{
    app::App,
    prelude::{Plugin, Commands, Camera2dBundle},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_camera);
    }
}

fn initialize_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
