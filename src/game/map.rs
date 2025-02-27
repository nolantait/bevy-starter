use bevy::prelude::*;
use crate::physics::Position;

struct Map {
    width: usize,
    height: usize,
}

#[derive(Component)]
#[require(Position)]
struct Tile;
