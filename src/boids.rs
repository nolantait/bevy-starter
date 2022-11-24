use bevy::{
    app::App,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;


use crate::input::MousePosition;

const BOID_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const BOID_SPEED: f32 = 200.;
const BOID_SIZE: f32 = 10.;

pub struct BoidPlugin;
struct BoidSpawned(Vec2);

#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Steering(Vec2);

#[derive(Component)]
struct Seek;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BoidSpawned>()
            .add_system(input_system)
            .add_system(seek_system.before(movement_system))
            .add_system(movement_system)
            .add_system(spawn_system);
    }
}

fn seek_system(
    mouse_position: Res<MousePosition>,
    mut query: Query<(&mut Steering, &Transform), With<Seek>>
) {
    for (mut steering, transform) in &mut query {
        let target = mouse_position.0;
        let position = transform.translation.truncate();
        let desired = (target - position).normalize_or_zero() * BOID_SPEED;
        steering.0 += desired;
    }
}


fn movement_system(
    mut query: Query<(&mut Velocity, &mut Steering, &mut Transform), With<Boid>>
) {
    for (mut velocity, mut steering, mut transform) in &mut query {
        let movement = steering.0;
        let rotation_angle = -movement.x.atan2(movement.y);
        transform.rotation = Quat::from_rotation_z(rotation_angle);

        velocity.linvel += movement;
        velocity.linvel = velocity.linvel.clamp_length_max(BOID_SPEED);
        steering.0 = Vec2::ZERO;
    }
}

fn input_system(
    buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut events: EventWriter<BoidSpawned>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let spawn_event = BoidSpawned(mouse_position.0);
        events.send(spawn_event);
    }
}


fn spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<BoidSpawned>
) {
    for spawn_event in events.iter() {
        let position = spawn_event.0;

        eprintln!("Spawning boid at {}", position);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(BOID_SIZE, 3).into()).into(),
                material: materials.add(ColorMaterial::from(BOID_COLOR)),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
            Boid,
            Seek,
            Steering(Vec2::ZERO),
            RigidBody::Dynamic,
            Velocity { linvel: Vec2::ZERO, angvel: 0. },
            Collider::ball(BOID_SIZE)
        ));
    }
}
