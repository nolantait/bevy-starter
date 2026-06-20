---
name: avian
description: Reference for Avian physics engine — rigid bodies, colliders, joints, spatial queries, collision events, character controllers, and debug rendering.
metadata:
  crate: avian3d
  bevy: "0.19"
---

## Setup

```toml
[dependencies]
avian2d = "0.7"   # 2D
avian3d = "0.7"   # 3D
```

```rust
use avian3d::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .run();
}
```

## Rigid bodies

Three types: `Dynamic` (simulated), `Kinematic` (programmatic), `Static` (immovable).

```rust
commands.spawn(RigidBody::Dynamic);
commands.spawn(RigidBody::Kinematic);
commands.spawn(RigidBody::Static);
```

## Position, Rotation, Transform

Avian syncs `Position` and `Rotation` with Bevy's `Transform` automatically.

```rust
fn move_things(mut q: Query<&mut Position>) {
    for mut pos in &mut q {
        pos.x += 1.0;
    }
}

fn rotate_things(mut q: Query<&mut Rotation>) {
    for mut rot in &mut q {
        *rot = rot.add_angle_fast(0.1);
    }
}
```

## Velocity

```rust
fn apply_velocity(mut q: Query<&mut LinearVelocity>) {
    for mut v in &mut q {
        v.x += 1.0;
    }
}

fn apply_angular_velocity(mut q: Query<&mut AngularVelocity>) {
    for mut av in &mut q {
        av.0 += 0.1;
    }
}
```

## Forces, impulses, and torque

```rust
use avian3d::dynamics::rigid_body::forces::*;

fn apply_forces(mut q: Query<(Entity, &mut ExternalForce, &mut ExternalTorque)>) {
    for (_, mut force, mut torque) in &mut q {
        force.apply_force(Vec3::Y * 10.0);
        torque.apply_torque(Vec3::Z * 2.0);
    }
}

fn apply_impulse(mut q: Query<&mut ExternalImpulse>) {
    for mut imp in &mut q {
        imp.apply_impulse(Vec3::Y * 5.0);
    }
}
```

Components: `ExternalForce`, `ExternalTorque`, `ExternalImpulse`, `ExternalAngularImpulse`.

## Colliders

```rust
commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(0.5),           // 3D sphere
    Collider::cuboid(1.0, 2.0, 1.0), // 3D box
    Collider::cylinder(0.5, 2.0),    // 3D cylinder
    Collider::capsule(0.5, 1.0),     // 3D capsule
    Collider::cone(0.5, 2.0),        // 3D cone
    Collider::halfspace(Vec3::Y),     // infinite plane
    Collider::compound(vec![]),      // compound of sub-shapes
));
```

2D colliders (`avian2d`): `Collider::circle(0.5)`, `Collider::rectangle(1.0, 2.0)`, etc.

### Collider from mesh

Requires `collider-from-mesh` feature (enabled by default):

```rust
Collider::from_bevy_mesh(&mesh, &ComputedTriangleMesh::default()).unwrap()
```

### Collider density

```rust
commands.spawn((RigidBody::Dynamic, Collider::sphere(0.5), ColliderDensity(1.0)));
```

## Friction and restitution

```rust
commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(0.5),
    Friction::new(0.3),
    Restitution::new(0.5),
));
```

## Collision layers

```rust
use avian3d::collision::CollisionLayer;

const PLAYER: CollisionLayer = CollisionLayer::from_bits(1 << 0);
const ENEMY: CollisionLayer = CollisionLayer::from_bits(1 << 1);
const WORLD: CollisionLayer = CollisionLayer::from_bits(1 << 2);

commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(0.5),
    CollisionLayers::new(PLAYER, WORLD | ENEMY), // member of PLAYER, collides with WORLD and ENEMY
));
```

## Sensors

Colliders with `Sensor` detect overlap without physical response:

```rust
commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(3.0),
    Sensor,
));
```

## Mass properties

```rust
commands.spawn((
    RigidBody::Dynamic,
    Collider::sphere(0.5),
    Mass(5.0),
    CenterOfMass::new(0.0, -0.5),
    LinearDamping(0.5),
    AngularDamping(0.5),
));
```

## Locked axes

Prevent translation/rotation along specific axes:

```rust
commands.spawn((
    RigidBody::Dynamic,
    LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_X,
));
```

## Gravity

```rust
fn zero_gravity(mut gravity: ResMut<Gravity>) {
    gravity.0 = Vec3::ZERO;
}

// Per-entity gravity scale:
commands.spawn((RigidBody::Dynamic, GravityScale(0.0))); // no gravity for this body
```

## Continuous Collision Detection (CCD)

Prevents tunneling for fast-moving objects:

```rust
use avian3d::dynamics::ccd::*;

commands.spawn((RigidBody::Dynamic, Ccd));
```

## Sleeping (auto-deactivation)

Bodies deactivate when at rest. Configure globally or per-entity:

```rust
// Disable sleeping for a specific body:
commands.spawn((RigidBody::Dynamic, SleepingDisabled));

// Default: bodies can sleep automatically.
```

## Collision events

### MessageReader (batch)

```rust
fn handle_collisions(mut start: MessageReader<CollisionStart>, mut end: MessageReader<CollisionEnd>) {
    for event in start.read() {
        info!("Collision started: {:?} vs {:?}", event.entity, event.other_entity());
    }
    for event in end.read() {
        info!("Collision ended: {:?} vs {:?}", event.entity, event.other_entity());
    }
}
```

Entities must have `CollisionEventsEnabled`:

```rust
commands.spawn((RigidBody::Dynamic, Collider::sphere(0.5), CollisionEventsEnabled));
```

### Observers (entity-specific)

```rust
commands
    .spawn((
        RigidBody::Dynamic,
        Collider::sphere(0.5),
        CollisionEventsEnabled,
    ))
    .observe(|_trigger: On<CollisionStart>| {
        info!("Collision started!");
    });
```

### Accessing collision data

```rust
fn read_collisions(mut collisions: Collisions) {
    for contacts in &mut collisions {
        info!("Contacts between {:?} and {:?}", contacts.entity1, contacts.entity2);
    }
}
```

## Joints

Joints spawn as separate entities referencing two bodies:

```rust
let body1 = commands.spawn(RigidBody::Dynamic).id();
let body2 = commands.spawn(RigidBody::Dynamic).id();

commands.spawn(FixedJoint::new(body1, body2));
commands.spawn(DistanceJoint::new(body1, body2));
commands.spawn(PrismaticJoint::new(body1, body2));
commands.spawn(RevoluteJoint::new(body1, body2));
commands.spawn(SphericalJoint::new(body1, body2));
```

### Joint configuration

```rust
commands.spawn((
    RevoluteJoint::new(body1, body2)
        .with_anchor(Vec3::new(5.0, 2.0, 0.0))
        .with_local_basis1(Quat::from_rotation_z(PI / 4.0)),
    JointDamping { linear: 0.1, angular: 0.1 },
));
```

### Disable collision between joined bodies

```rust
commands.spawn((FixedJoint::new(body1, body2), JointCollisionDisabled));
```

### Breaking joints

```rust
fn break_joints(query: Query<(Entity, &JointForces), Without<JointDisabled>>) {
    for (entity, forces) in &query {
        if forces.force().length() > 500.0 {
            commands.entity(entity).insert(JointDisabled);
        }
    }
}
```

## Spatial queries

### Raycasting

```rust
// Component-based (every frame):
commands.spawn(RayCaster::new(Vec3::ZERO, Dir3::X));

fn print_hits(query: Query<(&RayCaster, &RayHits)>) {
    for (ray, hits) in &query {
        for hit in hits.iter_sorted() {
            info!("Hit {:?} at {}", hit.entity, ray.origin + *ray.direction * hit.distance);
        }
    }
}

// SystemParam (one-off):
fn cast_ray(spatial: SpatialQuery) {
    let hit = spatial.cast_ray(Vec3::ZERO, Dir3::X, 100.0, true, &SpatialQueryFilter::default());
    if let Some(hit) = hit {
        info!("Hit {:?} at distance {}", hit.entity, hit.distance);
    }
}
```

### Shapecasting

```rust
commands.spawn(ShapeCaster::new(
    Collider::sphere(0.5),
    Vec3::ZERO,
    Quat::default(),
    Dir3::X,
));

fn print_shape_hits(query: Query<(&ShapeCaster, &ShapeHits)>) {
    for (_, hits) in &query {
        for hit in hits.iter() {
            info!("Shape hit {:?}", hit.entity);
        }
    }
}
```

### Spatial query filters

```rust
SpatialQueryFilter::default()
    .with_mask(CollisionLayer::from_bits(0xFF))
    .without_entities(&[exclude_entity]);
```

## Character controller

`MoveAndSlide` system parameter for kinematic character controllers:

```rust
use avian3d::character_controller::move_and_slide::*;

fn move_player(mut q: Query<(&mut LinearVelocity, &mut Transform), With<Player>>, time: Res<Time>) {
    let (mut velocity, _transform) = q.single_mut();
    let mut direction = Vec3::ZERO;
    // ... read input to set direction ...
    velocity.0 = direction * 10.0;

    // MoveAndSlide is applied automatically by the plugin if Present is added:
    commands.entity(player).insert(MoveAndSlide::default());
}
```

Available physics-based third-party crates: `bevy_ahoy` (kinematic), `bevy_tnua` (dynamic).

## Transform interpolation

Fixes choppy movement from fixed timestep physics:

```rust
// Per-entity:
commands.spawn((RigidBody::Dynamic, TransformInterpolation));

// Global default (all rigid bodies interpolated):
fn main() {
    App::new()
        .add_plugins(PhysicsPlugins::default()
            .set(PhysicsInterpolationPlugin::interpolate_all()))
        .run();
}
```

Components: `TransformInterpolation`, `TransformExtrapolation`, `TranslationInterpolation`, `RotationInterpolation`, etc.

## Debug rendering

```rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .run();
}
```

Requires the `debug-plugin` feature (enabled by default). Use `DebugRender::default()` to configure.

## Physics configuration

```rust
fn config(mut gravity: ResMut<Gravity>, mut substeps: ResMut<SubstepCount>) {
    gravity.0 = Vec3::new(0.0, -9.81, 0.0);
    substeps.0 = 10; // more substeps = more accurate but slower
}
```

## Pausing / stepping physics

```rust
fn pause(mut physics: ResMut<Physics>) {
    physics.paused = true;     // pause
    physics.paused = false;    // resume
    // Single-step while paused:
    if input.just_pressed(KeyCode::Space) {
        physics.schedule.stepped_state = StepState::StepOnce;
    }
}
```

## Diagnostics

```rust
use avian3d::diagnostics::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDiagnosticsPlugin, // requires bevy_diagnostic feature
        ))
        .run();
}
```

## Collision hooks

Filter or modify contacts with `CollisionHooks`:

```rust
use avian3d::collision::hooks::*;

#[derive(Resource)]
struct MyHooks;

impl CollisionHooks for MyHooks {
    fn filter_contact_pair(&self, _entities: (&Entity, &Entity)) -> Option<SolverFlags> {
        // return None to ignore, Some(...) to allow with flags
        Some(SolverFlags::default())
    }
    fn modify_contacts(&self, _contacts: &mut Vec<ContactData>, _entities: (&Entity, &Entity)) {}
}

fn main() {
    App::new()
        .add_plugins(PhysicsPluginsWithHooks::new(MyHooks))
        .run();
}
```

## CCD and speculative collision

```rust
commands.spawn((RigidBody::Dynamic, Ccd));
```

## Collider disabled

Temporarily disable a collider without removal:

```rust
commands.entity(entity).insert(ColliderDisabled);
```

## Rigid body disabled

```rust
commands.entity(entity).insert(RigidBodyDisabled);
```

## Sleep

Bodies at rest auto-sleep. Query sleeping state:

```rust
fn check_sleep(query: Query<&Sleeping>) {
    for sleeping in &query {
        info!("Entity is sleeping: {}", sleeping.is_sleeping());
    }
}
```

## Colliding entities

Get entities currently touching a collider:

```rust
fn check_touching(query: Query<&CollidingEntities>) {
    for colliding in &query {
        for entity in colliding.iter() {
            info!("Touching entity: {:?}", entity);
        }
    }
}
```

## Physics schedule

Systems run in `FixedPostUpdate` by default. Order custom systems relative to physics:

```rust
app.add_systems(FixedPostUpdate, my_system.after(PhysicsSystems::StepSimulation));
```

Available system sets: `PhysicsSystems::StepSimulation`, `PhysicsSystems::BroadPhase`, `PhysicsSystems::NarrowPhase`, `PhysicsSystems::Solver`, etc.
