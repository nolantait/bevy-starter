use bevy::prelude::*;

mod avian2d;
mod bevy_enhanced_input;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((avian2d::plugin, bevy_enhanced_input::plugin));
}
