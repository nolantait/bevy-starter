use super::*;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_camera);
    }
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
