use crate::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BULLET_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const BULLET_SPEED: f32 = 500.;
const BULLET_SIZE: Vec2 = Vec2::new(1., 1.);

#[derive(Component)]
struct Bullet;

#[derive(Bundle)]
struct BulletBundle {
    _bullet: Bullet,
    velocity: Velocity,
    physics: RigidBody,
    collider: Collider,
    gravity: GravityScale
}

impl Default for BulletBundle {
    fn default() -> Self {
        BulletBundle {
            _bullet: Bullet,
            physics: RigidBody::Dynamic,
            velocity: Velocity { linvel: Vec2::ZERO, angvel: 0. },
            collider: Collider::cuboid(BULLET_SIZE.x, BULLET_SIZE.y),
            gravity: GravityScale(0.)
        }
    }
}


pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_system);
    }
}

fn shoot_system(
    mut events: EventReader<Shoot>,
    query: Query<(&Transform, &Velocity), With<Boid>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _event in events.iter() {
        for (transform, velocity) in &query {
            let spawn_position = transform.translation + (velocity.linvel.normalize().extend(0.) * BOID_SIZE * 2.);
            let bullet_velocity = velocity.linvel.normalize() * BULLET_SPEED;

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::new(BULLET_SIZE).into()).into(),
                    material: materials.add(ColorMaterial::from(BULLET_COLOR)),
                    transform: Transform::from_translation(spawn_position),
                    ..default()
                },
                BulletBundle {
                    velocity: Velocity { linvel: bullet_velocity, ..default() },
                    ..default()
                }
            ));
        }
    }
}


