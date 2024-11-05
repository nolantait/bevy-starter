use bevy::prelude::*;

pub mod camera;
pub mod debug;
pub mod dev_tools;
pub mod game;
pub mod input;
pub mod physics;
pub mod utils;
pub mod window;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window::plugin,
            camera::plugin,
            physics::plugin,
            input::plugin,
            game::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((
            dev_tools::plugin,
            debug::plugin
        ));
    }
}
