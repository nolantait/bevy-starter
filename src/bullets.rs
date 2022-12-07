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
        app.add_system(shoot_system)
            .add_system(collision_system);
    }
}

fn shoot_system(
    mut events: EventReader<Shoot>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Transform, &Velocity), With<Boid>>,
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
                },
                ActiveEvents::COLLISION_EVENTS
            ));
        }
    }
}


fn collision_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut event_store: EventWriter<BoidShot>,
    boids: Query<&Boid>,
    bullets: Query<&Bullet>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _flags) = collision_event {
            eprintln!("Received collision event: {:?}", collision_event);
            eprintln!("E1: {:?}", e1);
            eprintln!("E2: {:?}", e2);

            if boids.get(*e1).is_ok() && boids.get(*e2).is_ok() {
                eprintln!("TWO BOIDS HIT EACH OTHER");
            }

            if boids.get(*e1).is_ok() && bullets.get(*e2).is_ok() {
                event_store.send(BoidShot {
                    boid: *e1,
                    bullet: *e2,
                })
            }

            if bullets.get(*e1).is_ok() && boids.get(*e2).is_ok() {
                event_store.send(BoidShot {
                    boid: *e2,
                    bullet: *e1
                })
            }
        }
    }
}
