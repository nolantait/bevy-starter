use super::*;

#[derive(Resource)]
pub struct MousePosition(pub Vec2);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MousePosition(Vec2::default()));
    }
}
