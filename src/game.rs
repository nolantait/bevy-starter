#![allow(unused)]

use avian2d::{math::*, prelude::*};
use bevy::{app::App, prelude::*};

use crate::utils::random_number;

// This is an example of the most simple plugin you can write, without
// having to implement any traits.
//
// If you wanted to toggle this plugin or configure it for the outside
// you can reach for a `PluginGroup`.

pub(super) fn plugin(app: &mut App) {
    // Your game logic here
    app;
}
