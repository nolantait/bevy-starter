use crate::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.25);
const LEVEL_COLOR: Color = Color::rgb(0.4, 0.4, 0.5);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugin(PhysicsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(InputPlugin)
            .add_startup_system(init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(40., 10., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(LEVEL_COLOR)),
            transform: Transform::default(),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(40., 10.)
    ));
}

