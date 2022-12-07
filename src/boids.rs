use crate::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;

// Constants
pub const BOID_SIZE: f32 = 10.;
const BOID_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BOID_SPEED: f32 = 250.;
const BOID_STEERING_FORCE: f32 = 0.75;
const BOID_SLOWING_RADIUS: f32 = 100.;
const BOID_AVOIDANCE_FACTOR: f32 = 100.;
pub const MAX_AVOIDANCE: f32 = 10000.;

pub enum Stance {
    Follow,
    Evade
}

// Events
pub struct BoidSpawned(pub Vec2);
pub struct StanceChanged(pub Stance);
pub struct Shoot;
pub struct BoidShot {
    pub boid: Entity,
    pub bullet: Entity
}

// Resources
#[derive(Resource)]
pub struct PlayerStance(pub Stance);

#[derive(Resource)]
pub struct AvoidanceFactor(pub f32);

// Components
#[derive(Component)]
pub struct Boid;

#[derive(Component)]
struct Steering(Vec2);

#[derive(Component)]
struct Seek;

#[derive(Component)]
struct Wander;

#[derive(Component)]
struct Avoid;

#[derive(Component)]
struct Flee;

// Bundles
#[derive(Bundle)]
struct BoidBundle {
    _boid: Boid,
    steering: Steering,
    physics: RigidBody,
    velocity: Velocity,
    collider: Collider,
    gravity: GravityScale,
}

impl Default for BoidBundle {
    fn default() -> Self {
        BoidBundle {
            _boid: Boid,
            steering: Steering(Vec2::ZERO),
            physics: RigidBody::Dynamic,
            velocity: Velocity { linvel: Vec2::ZERO, angvel: 0. },
            collider: Collider::ball(BOID_SIZE),
            gravity: GravityScale(0.)
        }
    }
}


// Plugin
pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerStance(Stance::Follow))
            .insert_resource(AvoidanceFactor(BOID_AVOIDANCE_FACTOR))
            .add_event::<BoidSpawned>()
            .add_event::<StanceChanged>()
            .add_event::<Shoot>()
            .add_event::<BoidShot>()
            .add_system(seek_system.before(movement_system))
            .add_system(wander_system.before(movement_system))
            .add_system(flee_system.before(movement_system))
            .add_system(avoidance_system.before(movement_system))
            .add_system(movement_system)
            .add_system(behaviour_system)
            .add_system(spawn_system);
    }
}



// Systems
fn avoidance_system(
    mut query: Query<(&mut Steering, &Transform), With<Avoid>>,
    avoidance_factor: Res<AvoidanceFactor>
) {
    let mut iterable = query.iter_combinations_mut();
    while let Some([
        (mut steering, transform), 
        (mut other_steering, other_transform)
    ]) = iterable.fetch_next() {
        let vector = other_transform.translation - transform.translation;
        let distance = vector.length_squared();
        let avoidance_force = (-vector.normalize().truncate() / distance) * avoidance_factor.0;

        steering.0 += avoidance_force;
        other_steering.0 += -avoidance_force;
    }
}

fn wander_system(
    mut query: Query<(&mut Steering, &Velocity), With<Wander>>
) {
    for (mut steering, velocity) in &mut query {
        let random_angle = random_number(-PI / 12., PI / 12.);
        let random_rotation = Quat::from_rotation_z(random_angle);
        let transform = Transform::from_xyz(velocity.linvel.x, velocity.linvel.y, 0.)
                                  .with_rotation(random_rotation);

        steering.0 += transform.rotation.mul_vec3(transform.translation).truncate().normalize_or_zero();
    }
}

fn flee_system(
    mouse_position: Res<MousePosition>,
    mut query: Query<(&mut Steering, &Transform, &Velocity), With<Flee>>
) {
    for (mut steering, transform, _velocity) in &mut query {
        let target = mouse_position.0;
        let position = transform.translation.truncate();
        let path_to_target = position - target;
        let distance = path_to_target.length();

        let mut desired_velocity = path_to_target.normalize_or_zero();
        if distance >= BOID_SLOWING_RADIUS {
            let arrival_force = BOID_SLOWING_RADIUS / distance;
            desired_velocity = desired_velocity * arrival_force;
        }

        steering.0 += desired_velocity;
    }
}

fn seek_system(
    mouse_position: Res<MousePosition>,
    mut query: Query<(&mut Steering, &Transform, &Velocity), With<Seek>>
) {
    for (mut steering, transform, _velocity) in &mut query {
        let target = mouse_position.0;
        let position = transform.translation.truncate();
        let path_to_target = target - position;
        let distance = path_to_target.length();

        let mut desired_velocity = path_to_target.normalize_or_zero();
        if distance <= BOID_SLOWING_RADIUS {
            let arrival_force = distance / BOID_SLOWING_RADIUS;
            desired_velocity = desired_velocity * arrival_force;
        }

        steering.0 += desired_velocity;
    }
}

fn movement_system(
    mut query: Query<(&mut Velocity, &mut Steering, &mut Transform), With<Boid>>
) {
    for (mut velocity, mut steering, mut transform) in &mut query {
        let steer_force = steering.0 * BOID_STEERING_FORCE * BOID_SPEED;
        let desired_velocity = steer_force - velocity.linvel;
        velocity.linvel += desired_velocity;
        velocity.linvel = velocity.linvel.clamp_length_max(BOID_SPEED);

        let rotation_angle = -velocity.linvel.x.atan2(velocity.linvel.y);
        transform.rotation = Quat::from_rotation_z(rotation_angle);

        // Reset steering force for next tick
        steering.0 = Vec2::ZERO;
    }
}



fn behaviour_system(
    mut events: EventReader<StanceChanged>,
    mut commands: Commands,
    query: Query<Entity, With<Boid>>,
    mut stance: ResMut<PlayerStance>
) {
    for event in events.iter() {
        match event.0 {
            Stance::Follow => {
                for entity in &query {
                    commands.entity(entity).remove::<Flee>();
                    commands.entity(entity).insert(Seek);
                }
                stance.0 = Stance::Follow;
            },
            Stance::Evade => {
                for entity in &query {
                    commands.entity(entity).remove::<Seek>();
                    commands.entity(entity).insert(Flee);
                }
                stance.0 = Stance::Evade;
            }
        }

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
            BoidBundle::default(),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(BOID_SIZE, 3).into()).into(),
                material: materials.add(ColorMaterial::from(BOID_COLOR)),
                transform: Transform::from_xyz(position.x, position.y, 0.),
                ..default()
            },
            Avoid,
            Wander
        ));
    }
}

