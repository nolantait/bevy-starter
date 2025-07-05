//! Font assets.

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<FontAssets>();

    let assets = app.world().resource::<AssetServer>();
    let default_font: Handle<Font> = assets.load("fonts/PixelSmall.ttf");

    app.insert_resource(FontAssets {
        default: default_font,
    });
}

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub(crate) struct FontAssets {
    pub default: Handle<Font>,
}
