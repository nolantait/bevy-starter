#![allow(unused_imports)]

use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod utils;

/// Use this module instead of importing the `components`, `plugins`, `resources`, and `utils`
/// modules directly.
mod prelude {
    pub use super::*;
    pub use {components::*, plugins::*, resources::*, utils::*};
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            plugins::asset_tracking::plugin,
            plugins::default::plugin,
            plugins::fonts::plugin,
            plugins::camera::plugin,
            plugins::physics::plugin,
            plugins::input::plugin,
            plugins::game::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins((plugins::dev_tools::plugin, plugins::debug::plugin));
    }
}
