#![allow(unused)]

use avian2d::{math::*, prelude::*};
use bevy::{app::App, prelude::*, sprite::MaterialMesh2dBundle};

use crate::utils::random_number;

pub struct GamePlugin;

#[derive(Component)]
struct Boid {
    steering: Vec2,
}

impl Boid {
    fn new() -> Self {
        Self {
            steering: Vec2::default(),
        }
    }

    fn wander(&mut self) {
        let angle = random_number(0., 2. * PI);
        let radius = 10.;

        self.steering += Vec2::new(angle.cos() * radius, angle.sin() * radius);

        self.steering = self.steering.normalize() * radius;
    }

    fn seek(&mut self, current_position: Vec2, current_velocity: Vec2, target: Vec2) {
        let desired = target - current_position;
        let speed = 10.;
        let velocity = desired.normalize() * speed;
        self.steering = velocity - current_velocity;
    }
}

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

    let color = Color::srgb(1., 0., 0.);

    let mesh = meshes.add(triangle);
    let material = materials.add(color);

    commands.spawn((
        Boid {
            steering: Vec2::default(),
        },
        RigidBody::Dynamic,
        LinearVelocity::default(),
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            transform: Transform::default(),
            ..default()
        },
    ));
}

fn wander_boid(mut query: Query<&mut Boid>) {
    for mut boid in &mut query {
        boid.wander();
    }
}

fn move_boid(mut query: Query<(&mut LinearVelocity, &Boid)>) {
    for (mut velocity, boid) in &mut query {
        velocity.x += boid.steering.x;
        velocity.y += boid.steering.y;
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_boid)
        .add_systems(Update, wander_boid)
        .add_systems(Update, move_boid);
}
