use bevy::{app::App, prelude::*, sprite::MaterialMesh2dBundle};

pub struct GamePlugin;

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Velocity {
  linvel: Vec2,
  angvel: Vec2,
}


const BOID_SIZE: f32 = 5.;

fn spawn_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(BOID_SIZE));
    let color = ColorMaterial::from(Color::srgb(1., 0., 0.));

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);

    commands.spawn((
        Boid,
        // RigidBody::Dynamic,
        // Velocity {
        //     linvel: Vec2::new(0., 5.0),
        //     angvel: 0.4,
        // },
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

fn move_boid(mut query: Query<&mut Velocity, With<Boid>>) {
    for mut velocity in &mut query {
        velocity.linvel += 1.5;
        velocity.angvel += 99.5;
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boid)
            .add_systems(Update, move_boid);
    }
}
