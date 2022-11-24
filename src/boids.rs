use bevy::{
    app::App,
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use crate::input::MousePosition;
use crate::random_number;
use std::f32::consts::PI;

// Constants
const BOID_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const BOID_SPEED: f32 = 200.;
const BOID_SIZE: f32 = 10.;
const BOID_STEERING_FORCE: f32 = 20.;
const BOID_SLOWING_RADIUS: f32 = 100.;

pub struct BoidPlugin;

// Events
struct BoidSpawned(Vec2);

// Components
#[derive(Component)]
struct Boid;

#[derive(Component)]
struct Steering(Vec2);

#[derive(Component)]
struct Seek;

#[derive(Component)]
struct Wander;

// Plugin
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BoidSpawned>()
            .add_system(input_system)
            .add_system(seek_system.before(movement_system))
            .add_system(wander_system.before(movement_system))
            .add_system(movement_system)
            .add_system(spawn_system);
    }
}


// Systems
fn wander_system(
    mut query: Query<(&mut Steering, &Velocity), With<Wander>>
) {
    for (mut steering, velocity) in &mut query {
        let circle_center = velocity.linvel.normalize_or_zero() * BOID_SPEED;
        let rotation = Quat::from_rotation_z(random_number(-PI, PI));
        let displacement = (Vec2::Y * BOID_SPEED).extend(0.);
        let wandering_force = rotation.mul_vec3(displacement).truncate();
        steering.0 = circle_center + wandering_force;
    }
}

fn seek_system(
    mouse_position: Res<MousePosition>,
    mut query: Query<(&mut Steering, &Transform, &Velocity), With<Seek>>
) {
    for (mut steering, transform, velocity) in &mut query {
        let target = mouse_position.0;
        let position = transform.translation.truncate();
        let path_to_target = target - position;
        let distance = path_to_target.length();

        let mut desired_velocity = path_to_target.normalize_or_zero();
        if distance <= BOID_SLOWING_RADIUS {
            let arrival_force = distance / BOID_SLOWING_RADIUS;
            desired_velocity = desired_velocity * BOID_SPEED * arrival_force;
        } else {
            desired_velocity = desired_velocity * BOID_SPEED;
        }

        steering.0 += desired_velocity - velocity.linvel;
    }
}


fn movement_system(
    mut query: Query<(&mut Velocity, &mut Steering, &mut Transform), With<Boid>>
) {
    for (mut velocity, mut steering, mut transform) in &mut query {
        velocity.linvel += steering.0.clamp_length_max(BOID_STEERING_FORCE);
        velocity.linvel = velocity.linvel.clamp_length_max(BOID_SPEED);

        let rotation_angle = -velocity.linvel.x.atan2(velocity.linvel.y);
        transform.rotation = Quat::from_rotation_z(rotation_angle);

        // Reset steering force for next tick
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

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(BOID_SIZE, 3).into()).into(),
                material: materials.add(ColorMaterial::from(BOID_COLOR)),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
            Boid,
            Wander,
            Steering(Vec2::ZERO),
            RigidBody::Dynamic,
            Velocity { linvel: Vec2::ZERO, angvel: 0. },
            Collider::ball(BOID_SIZE)
        ));
    }
}
