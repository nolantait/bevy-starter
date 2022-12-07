use bevy::prelude::*;

mod debug;
mod physics;
mod camera;
mod input;
mod game;
mod utils;

mod prelude {
    pub use crate::debug::*;
    pub use crate::physics::*;
    pub use crate::camera::*;
    pub use crate::input::*;
    pub use crate::game::*;
    pub use crate::utils::*;
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
}

use crate::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(GamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}


