use bevy::prelude::*;

mod camera;
mod debug;
mod dev_tools;
mod game;
mod input;
mod physics;
mod utils;
mod window;
mod pathfinding;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            window::plugin,
            camera::plugin,
            physics::plugin,
            pathfinding::plugin,
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
