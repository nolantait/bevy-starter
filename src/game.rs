use avian2d::{math::*, prelude::*};
use bevy::{app::App, prelude::*, sprite::MaterialMesh2dBundle};

pub struct GamePlugin;

#[derive(Component)]
struct Boid;

const BOID_SIZE: f32 = 5.;

fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let triangle = Triangle2d::new(
        Vec2::new(0.0, BOID_SIZE),
        Vec2::new(-BOID_SIZE, -BOID_SIZE),
        Vec2::new(BOID_SIZE, -BOID_SIZE),
    );

    let color = ColorMaterial::from(Color::srgb(1., 0., 0.));

    let mesh_handle = meshes.add(triangle);
    let material_handle = materials.add(color);

    commands.spawn((
        Boid,
        RigidBody::Dynamic,
        LinearVelocity::default(),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

fn move_boid(mut query: Query<&mut LinearVelocity, With<Boid>>) {
    for mut velocity in &mut query {
        velocity.x += 0.05;
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boid)
            .add_systems(Update, move_boid);
    }
}
