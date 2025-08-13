#![allow(unused_imports)]
#![allow(dead_code)]

use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod styles;
mod utils;

/// Use this module instead of importing the `components`, `plugins`, `resources`, and `utils`
/// modules directly.
mod prelude {
    pub use super::*;
    pub use {components::*, plugins::*, resources::*, styles::*, utils::*};

    // Preparing for Bevy 0.17
    // https://hackmd.io/@bevy/BkTCu5NElx
    pub type On<'w, E, B> = Trigger<'w, E, B>;
    pub type Add = OnAdd;
    pub type Insert = OnInsert;
    pub type Replace = OnReplace;
    pub type Remove = OnRemove;
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
