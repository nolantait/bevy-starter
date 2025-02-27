use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Position(Vec2);

impl Position {
    fn new(x: f32, y: f32) -> Self {
        Position(Vec2::new(x, y))
    }

    fn lerp(&mut self, target: Vec2, alpha: f32) {
        self.0 = self.0.lerp(target, alpha);
    }
}

#[derive(Component)]
struct TargetPosition(Vec2);

fn move_towards_target_position(
    mut query: Query<(&mut Position, &TargetPosition)>,
    time: Res<Time>,
) {
    for (mut position, target_position) in query.iter_mut() {
        let alpha = 0.1 * time.delta_secs();
        position.lerp(target_position.0, alpha);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, move_towards_target_position);
}
