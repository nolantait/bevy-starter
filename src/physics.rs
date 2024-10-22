use super::*;
use avian2d::{math::*, prelude::*};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels.
        // The unit allows the engine to tune its parameters for the scale of the world, improving stability.
        app
            .add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
            .insert_resource(Gravity(Vector::NEG_Y * 1000.0));
    }
}
