use bevy::prelude::*;

mod asset_tracking;
mod camera;
mod debug;
mod default;
mod dev_tools;
mod fonts;
mod game;
mod input;
mod physics;
mod utils;

mod prelude {
    pub use crate::utils::*;
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            asset_tracking::plugin,
            default::plugin,
            fonts::plugin,
            camera::plugin,
            physics::plugin,
            input::plugin,
            game::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((dev_tools::plugin, debug::plugin));
    }
}
